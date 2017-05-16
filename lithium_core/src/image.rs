use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter;
use std::cmp::min;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender, Receiver};
use {Color, Vec2, Rect, Id};
use util::IdIdentityHasherBuilder;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct ImageId(Id);

const CELL_SIZE: i32 = 64;

#[derive(Debug)]
pub struct Image {
	id: ImageId,

	size: Vec2<i32>,
	pixels: Vec<Color>,

	cells: Vec<u64>,
	delete_listeners: Mutex<Vec<Sender<ImageId>>>,
}

pub struct ImagesWatcher<T> {
	images: HashMap<ImageId, ImageWatcher<T>, IdIdentityHasherBuilder>,
	deleted_receiver: Receiver<ImageId>,
	deleted_sender: Sender<ImageId>,
}

struct ImageWatcher<T> {
	cells: Vec<u64>,
	data: T,
}

impl Clone for Image {
	fn clone(&self) -> Self {
		Image::new(self.size, self.pixels.clone())
	}
}

impl Image {
	pub fn new(size: Vec2<i32>, pixels: Vec<Color>) -> Self {
		Image {
			id: ImageId(Id::unique()),
			size,
			pixels,
			cells: Vec::new(),
			delete_listeners: Mutex::new(Vec::new()),
		}
	}

	pub fn size(&self) -> Vec2<i32> {
		self.size
	}

	pub fn update(&mut self, region: Option<Rect<i32>>, data: &[Color]) {	
		let region = region.unwrap_or(Rect::from_top_left_and_size(Vec2::zero(), self.size));
		self.invalidate(region);		

		for y in region.top..region.bottom {			
			for x in region.left..region.right {
				let index = self.index(Vec2::new(x, y));
				self.pixels[index] = data[((y - region.top)*region.width() + x - region.bottom) as usize];
			}
		}
	}

	pub fn update_rgb24(&mut self, region: Option<Rect<i32>>, data: &[[u8; 3]]) {
		let region = region.unwrap_or(Rect::from_top_left_and_size(Vec2::zero(), self.size));
		self.invalidate(region);

		for y in region.top..region.bottom {
			for x in region.left..region.right {
				let index = self.index(Vec2::new(x, y));

				let new_pixel = data[((y - region.top)*region.width() + x - region.bottom) as usize];
				let new_pixel = Color::from_rgb24(new_pixel[0], new_pixel[1], new_pixel[2]);

				self.pixels[index] = new_pixel;
			}
		}
	}

	pub fn update_rgba32(&mut self, region: Option<Rect<i32>>, data: &[[u8; 4]]) {
		let region = region.unwrap_or(Rect::from_top_left_and_size(Vec2::zero(), self.size));
		self.invalidate(region);

		for y in region.top..region.bottom {
			for x in region.left..region.right {
				let index = self.index(Vec2::new(x, y));

				let new_pixel = data[((y - region.top)*region.width() + x - region.bottom) as usize];
				let new_pixel = Color::from_rgba32(new_pixel[0], new_pixel[1], new_pixel[2], new_pixel[3]);

				self.pixels[index] = new_pixel;
			}
		}
	}

	pub fn index(&self, position: Vec2<i32>) -> usize {
		(position.y*self.size.x + position.x) as usize
	}

	pub fn pixel(&self, position: Vec2<i32>) -> Color {
		self.pixels[self.index(position)]
	}

	pub fn pixel_mut(&mut self, position: Vec2<i32>) -> &mut Color {
		let index = self.index(position);
		self.invalidate(Rect::from_top_left_and_size(position, Vec2::new(1, 1)));
		&mut self.pixels[index]
	}

	pub fn pixels(&self) -> &[Color] {
		&self.pixels
	}

	pub fn pixels_mut(&mut self, invalidation_region: Option<Rect<i32>>) -> &mut [Color] {
		let region = invalidation_region.unwrap_or(Rect::from_top_left_and_size(Vec2::zero(), self.size));
		self.invalidate(region);
		&mut self.pixels
	}

	fn watch<T>(&self, data: T, delete_listenter: Sender<ImageId>) -> ImageWatcher<T> {
		let mut listeners = self.delete_listeners.lock().unwrap_or_else(|e| e.into_inner());
		listeners.push(delete_listenter);

		ImageWatcher {
			cells: self.cells.clone(),
			data,
		}
	}

	fn invalidate(&mut self, region: Rect<i32>) {
		let cells_row_len = (self.size.x - 1)/CELL_SIZE + 1;
		let cells_column_len = (self.size.x - 1)/CELL_SIZE + 1;

		if self.cells.is_empty() {
			self.cells = iter::repeat(0).take((cells_row_len*cells_column_len) as usize).collect();
		}

		for y in (region.top/CELL_SIZE)..(((region.bottom - 1)/CELL_SIZE) + 1) {
			for x in (region.left/CELL_SIZE)..(((region.right - 1)/CELL_SIZE) + 1) {
				let index = (y*cells_row_len + x) as usize;
				self.cells[index] = self.cells[index].wrapping_add(1);
			}
		}
	}
}

impl Drop for Image {
	fn drop(&mut self) {
		let listeners = self.delete_listeners.get_mut().unwrap_or_else(|e| e.into_inner());
		for listener in listeners {
			let _ = listener.send(self.id);
		}
	}
}

pub enum ImageState<T> {
	New(T),
	Same(T),
	Updated(T, Rect<i32>),
}

impl<T> ImagesWatcher<T> {
	pub fn new() -> Self {
		let (deleted_sender, deleted_receiver) = channel();

		ImagesWatcher {
			images: HashMap::with_hasher(IdIdentityHasherBuilder),
			deleted_receiver,
			deleted_sender,
		}
	}

	pub fn get_image_state<F>(&mut self, image: &Image, constructor: F) -> ImageState<T>
	  where
	    F: FnOnce() -> T,
		T: Copy,
	{
		match self.images.entry(image.id) {
			Entry::Vacant(entry) => {
				let data = constructor();
				entry.insert(image.watch(data, self.deleted_sender.clone()));

				ImageState::New(data)
			}
			Entry::Occupied(mut entry) => {
				let watcher = entry.get_mut();
				if let Some(region) = watcher.get_and_reset_invalid_region(image) {
					ImageState::Updated(watcher.data, region)
				} else {
					ImageState::Same(watcher.data)
				}
			}
		}
	}

	pub fn deleted<'a>(&'a mut self) -> Box<Iterator<Item=T> + 'a> {
		let images = &mut self.images;

		Box::new(self.deleted_receiver.try_iter().map(move |image_id| {
			images.remove(&image_id).unwrap().data
		}))
	}
}

impl<T> ImageWatcher<T> {
	fn get_and_reset_invalid_region(&mut self, image: &Image) -> Option<Rect<i32>> {
		if image.cells == self.cells {
			None
		} else {
			let mut region: Option<Rect<i32>> = None;

			let cells_row_len = (image.size.x - 1)/CELL_SIZE + 1;

			for y in 0..(((image.size.y - 1)/CELL_SIZE) + 1) {
				for x in 0..(((image.size.x - 1)/CELL_SIZE) + 1) {
					let index = (y*cells_row_len + x) as usize;
					if self.cells[index] != image.cells[index] {
						if let Some(ref mut region) = region {
							*region |= Rect::from_top_left_and_size(Vec2::new(x, y), Vec2::new(1, 1));
						}
					}
				}
			}

			region.map(|region| {
				Rect {
					left: region.left*CELL_SIZE,
					top: region.top*CELL_SIZE,
					right: min(region.right*CELL_SIZE, image.size.x),
					bottom: min(region.bottom*CELL_SIZE, image.size.x),
				}
			})
		}
	}
}
