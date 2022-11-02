use glam::Vec2;

pub(crate) fn clear_frame(color: [u8;4], frame: &mut[u8])
{
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate()
    {
        pixel.copy_from_slice(&color);
    }
}

pub(crate) fn draw_square(pos: Vec2, size: Vec2, frame: &mut[u8], width: u32)
{
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % width as usize) as i16;
        let y = (i / width as usize) as i16;

        let inside_the_box = x >= pos.x as i16
            && x < pos.x as i16 + size.x as i16
            && y >= pos.y as i16
            && y < pos.y as i16 + size.y as i16;

        let rgba = if inside_the_box {
            [0x5e, 0x48, 0xe8, 0xff]
        } else
        {
            [pixel[0], pixel[1], pixel[2], pixel[3]]
        };

        pixel.copy_from_slice(&rgba);
    }
}

pub fn draw_line(from: Vec2, to: Vec2){}