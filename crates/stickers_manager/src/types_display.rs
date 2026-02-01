// Fixed Display implementations

impl fmt::Display for Sticker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sticker(set_id={}, file_id={}, format={}, type={:?})",
            self.set_id.get(),
            self.file_id.get(),
            self.format,
            self.sticker_type
        )
    }
}

impl fmt::Display for StickerSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StickerSet(id={}, name='{}', title='{}', count={})",
            self.id.get(),
            self.short_name,
            self.title,
            self.sticker_count
        )
    }
}
