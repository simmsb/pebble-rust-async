pub use crate::bindings::GColor;

impl GColor {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> GColor {
        let argb: u8 = (3u8 << 6) | ((r & 0b11) << 4) | ((g & 0b11) << 2) | (b & 0b11);
        GColor { argb }
    }

    pub const fn from_hex(hex: u32) -> GColor {
        Self::from_rgb(
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }

    pub const CLEAR: GColor = GColor { argb: 0b00000000 };
    pub const BLACK: GColor = GColor { argb: 0b11000000 };
    pub const OXFORD_BLUE: GColor = GColor { argb: 0b11000001 };
    pub const DUKE_BLUE: GColor = GColor { argb: 0b11000010 };
    pub const BLUE: GColor = GColor { argb: 0b11000011 };
    pub const DARK_GREEN: GColor = GColor { argb: 0b11000100 };
    pub const MIDNIGHT_GREEN: GColor = GColor { argb: 0b11000101 };
    pub const COBALT_BLUE: GColor = GColor { argb: 0b11000110 };
    pub const BLUE_MOON: GColor = GColor { argb: 0b11000111 };
    pub const ISLAMIC_GREEN: GColor = GColor { argb: 0b11001000 };
    pub const JAEGER_GREEN: GColor = GColor { argb: 0b11001001 };
    pub const TIFFANY_BLUE: GColor = GColor { argb: 0b11001010 };
    pub const VIVID_CERULEAN: GColor = GColor { argb: 0b11001011 };
    pub const GREEN: GColor = GColor { argb: 0b11001100 };
    pub const MALACHITE: GColor = GColor { argb: 0b11001101 };
    pub const MEDIUM_SPRING_GREEN: GColor = GColor { argb: 0b11001110 };
    pub const CYAN: GColor = GColor { argb: 0b11001111 };
    pub const BULGARIAN_ROSE: GColor = GColor { argb: 0b11010000 };
    pub const IMPERIAL_PURPLE: GColor = GColor { argb: 0b11010001 };
    pub const INDIGO: GColor = GColor { argb: 0b11010010 };
    pub const ELECTRIC_ULTRAMARINE: GColor = GColor { argb: 0b11010011 };
    pub const ARMY_GREEN: GColor = GColor { argb: 0b11010100 };
    pub const DARK_GRAY: GColor = GColor { argb: 0b11010101 };
    pub const LIBERTY: GColor = GColor { argb: 0b11010110 };
    pub const VERY_LIGHT_BLUE: GColor = GColor { argb: 0b11010111 };
    pub const KELLY_GREEN: GColor = GColor { argb: 0b11011000 };
    pub const MAY_GREEN: GColor = GColor { argb: 0b11011001 };
    pub const CADET_BLUE: GColor = GColor { argb: 0b11011010 };
    pub const PICTON_BLUE: GColor = GColor { argb: 0b11011011 };
    pub const BRIGHT_GREEN: GColor = GColor { argb: 0b11011100 };
    pub const SCREAMIN_GREEN: GColor = GColor { argb: 0b11011101 };
    pub const MEDIUM_AQUAMARINE: GColor = GColor { argb: 0b11011110 };
    pub const ELECTRIC_BLUE: GColor = GColor { argb: 0b11011111 };
    pub const DARK_CANDY_APPLE_RED: GColor = GColor { argb: 0b11100000 };
    pub const JAZZBERRY_JAM: GColor = GColor { argb: 0b11100001 };
    pub const PURPLE: GColor = GColor { argb: 0b11100010 };
    pub const VIVID_VIOLET: GColor = GColor { argb: 0b11100011 };
    pub const WINDSOR_TAN: GColor = GColor { argb: 0b11100100 };
    pub const ROSE_VALE: GColor = GColor { argb: 0b11100101 };
    pub const PURPUREUS: GColor = GColor { argb: 0b11100110 };
    pub const LAVENDER_INDIGO: GColor = GColor { argb: 0b11100111 };
    pub const LIMERICK: GColor = GColor { argb: 0b11101000 };
    pub const BRASS: GColor = GColor { argb: 0b11101001 };
    pub const LIGHT_GRAY: GColor = GColor { argb: 0b11101010 };
    pub const BABY_BLUE_EYES: GColor = GColor { argb: 0b11101011 };
    pub const SPRING_BUD: GColor = GColor { argb: 0b11101100 };
    pub const INCHWORM: GColor = GColor { argb: 0b11101101 };
    pub const MINT_GREEN: GColor = GColor { argb: 0b11101110 };
    pub const CELESTE: GColor = GColor { argb: 0b11101111 };
    pub const RED: GColor = GColor { argb: 0b11110000 };
    pub const FOLLY: GColor = GColor { argb: 0b11110001 };
    pub const FASHION_MAGENTA: GColor = GColor { argb: 0b11110010 };
    pub const MAGENTA: GColor = GColor { argb: 0b11110011 };
    pub const ORANGE: GColor = GColor { argb: 0b11110100 };
    pub const SUNSET_ORANGE: GColor = GColor { argb: 0b11110101 };
    pub const BRILLIANT_ROSE: GColor = GColor { argb: 0b11110110 };
    pub const SHOCKING_PINK: GColor = GColor { argb: 0b11110111 };
    pub const CHROME_YELLOW: GColor = GColor { argb: 0b11111000 };
    pub const RAJAH: GColor = GColor { argb: 0b11111001 };
    pub const MELON: GColor = GColor { argb: 0b11111010 };
    pub const RICH_BRILLIANT_LAVENDER: GColor = GColor { argb: 0b11111011 };
    pub const YELLOW: GColor = GColor { argb: 0b11111100 };
    pub const ICTERINE: GColor = GColor { argb: 0b11111101 };
    pub const PASTEL_YELLOW: GColor = GColor { argb: 0b11111110 };
    pub const WHITE: GColor = GColor { argb: 0b11111111 };
}
