use skia_safe::{Color, Color4f, Data, EncodedImageFormat, Font, FontMgr, FontStyle, Paint, PaintStyle, Path, Rect, Surface, TextBlob, Typeface, paint, surfaces, utils::text_utils::Align};
use std::mem;

pub struct Canvas {
    surface: Surface,
    path: Path,
    paint: Paint,
}

pub fn default_typeface() -> Typeface {
    let font_mgr = FontMgr::new();
        font_mgr
            .legacy_make_typeface(None, FontStyle::default())
            .unwrap()
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let mut surface = surfaces::raster_n32_premul((width, height)).expect("surface");
        let path = Path::new();
        let mut paint = Paint::default();
        paint.set_color(Color::from_argb(0xFF, 0xFF, 0x00, 0x00));
        paint.set_anti_alias(true);
        paint.set_stroke_width(1.0);
        surface.canvas().clear(Color::WHITE);
        Canvas {
            surface,
            path,
            paint,
        }
    }

    #[inline]
    pub fn save(&mut self) {
        self.canvas().save();
    }

    #[inline]
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.canvas().translate((dx, dy));
    }

    #[inline]
    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.canvas().scale((sx, sy));
    }

    #[inline]
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.begin_path();
        self.path.move_to((x, y));
    }

    #[inline]
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to((x, y));
    }

    #[inline]
    pub fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.path.quad_to((cpx, cpy), (x, y));
    }

    #[allow(dead_code)]
    #[inline]
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path.cubic_to((cp1x, cp1y), (cp2x, cp2y), (x, y));
    }

    #[allow(dead_code)]
    #[inline]
    pub fn close_path(&mut self) {
        self.path.close();
    }

    #[inline]
    pub fn begin_path(&mut self) {
        let new_path = Path::new();
        self.surface.canvas().draw_path(&self.path, &self.paint);
        let _ = mem::replace(&mut self.path, new_path);
    }

    #[inline]
    pub fn stroke(&mut self) {
        self.paint.set_style(PaintStyle::Stroke);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn fill(&mut self) {
        self.paint.set_style(PaintStyle::Fill);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn set_line_width(&mut self, width: f32) {
        self.paint.set_stroke_width(width);
    }

    #[inline]
    pub fn set_text(&mut self, value: &str) {


        let fill_paint = &mut Paint::default();
        let stroke_paint = &mut Paint::default();
        stroke_paint
            .set_style(paint::Style::Stroke)
            .set_stroke_width(3.0);

        let _ = &self.canvas()
            .draw_rect(Rect::from_point_and_size((10, 10), (60, 20)), fill_paint)
            .draw_rect(Rect::from_point_and_size((80, 10), (60, 20)), stroke_paint);

        stroke_paint.set_stroke_width(5.0);
        let _ = &self.canvas().draw_oval(Rect::from_point_and_size((150, 10), (60, 20)), stroke_paint);

        let blob = TextBlob::from_str("SKIA", &Font::from_typeface(default_typeface(), 80.0)).unwrap();

        fill_paint.set_color(Color::from_argb(0xFF, 0xFF, 0x00, 0x00));
        let _ = &self.canvas().draw_text_blob(&blob, (20, 120), fill_paint);

        fill_paint.set_color(Color::from_argb(0xFF, 0x00, 0x00, 0xFF));
        let _ = &self.canvas().draw_text_blob(&blob, (20, 220), fill_paint);
    }

    #[inline]
    pub fn data(&mut self) -> Data {
        let image = self.surface.image_snapshot();
        let mut context = self.surface.direct_context();
        image
            .encode(context.as_mut(), EncodedImageFormat::PNG, None)
            .unwrap()
    }

    #[inline]
    fn canvas(&mut self) -> &skia_safe::Canvas {
        self.surface.canvas()
    }
}
