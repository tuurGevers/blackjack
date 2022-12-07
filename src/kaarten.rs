pub mod kaarten {
    use rand::distributions::{Distribution, Standard};
    use rand::Rng;
    use std::fmt::{Display, Formatter};
    use std::io;

    #[derive(Clone, PartialEq)]
    enum KaartKleur {
        HARTEN,
        SCHOPPEN,
        RUITEN,
        KLAVEREN,
    }

    #[derive(Clone, PartialEq)]
    enum KaartWaarde {
        AAS,
        TWEE,
        DRIE,
        VIER,
        VIJF,
        ZES,
        ZEVEN,
        ACHT,
        NEGEN,
        TIEN,
        BOER,
        DAME,
        HEER,
    }

    impl Distribution<KaartWaarde> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> KaartWaarde {
            match rng.gen_range(1..14) {
                2 => KaartWaarde::TWEE,
                3 => KaartWaarde::DRIE,
                4 => KaartWaarde::VIER,
                5 => KaartWaarde::VIJF,
                6 => KaartWaarde::ZES,
                7 => KaartWaarde::ZEVEN,
                8 => KaartWaarde::ACHT,
                9 => KaartWaarde::NEGEN,
                10 => KaartWaarde::TIEN,
                11 => KaartWaarde::BOER,
                12 => KaartWaarde::DAME,
                13 => KaartWaarde::HEER,
                _ => KaartWaarde::AAS,
            }
        }
    }

    impl Display for KaartWaarde {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                KaartWaarde::TWEE => write!(f, "Twee"),
                KaartWaarde::DRIE => write!(f, "Drie"),
                KaartWaarde::VIER => write!(f, "Vier"),
                KaartWaarde::VIJF => write!(f, "Vijf"),
                KaartWaarde::ZES => write!(f, "Zes"),
                KaartWaarde::ZEVEN => write!(f, "Zeven"),
                KaartWaarde::ACHT => write!(f, "Acht"),
                KaartWaarde::NEGEN => write!(f, "Negen"),
                KaartWaarde::TIEN => write!(f, "Tien"),
                KaartWaarde::BOER => write!(f, "Boer"),
                KaartWaarde::DAME => write!(f, "Dame"),
                KaartWaarde::HEER => write!(f, "Heer"),
                KaartWaarde::AAS => write!(f, "Aas"),
            }
        }
    }

    impl Display for KaartKleur {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                KaartKleur::HARTEN => write!(f, "Harten"),
                KaartKleur::RUITEN => write!(f, "Ruiten"),
                KaartKleur::SCHOPPEN => write!(f, "Schoppen"),
                KaartKleur::KLAVEREN => write!(f, "Klaveren"),
            }
        }
    }

    impl Distribution<KaartKleur> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> KaartKleur {
            match rng.gen_range(1..5) {
                2 => KaartKleur::HARTEN,
                3 => KaartKleur::RUITEN,
                4 => KaartKleur::SCHOPPEN,
                _ => KaartKleur::KLAVEREN,
            }
        }
    }

    impl Display for Kaart {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{} {}",
                self.kaart_kleur.to_string(),
                self.kaart_waarde.to_string()
            )
        }
    }

    pub struct StokKaarten {
        getrokken: Vec<Kaart>,
        kaart_aantal: u8,
    }

    #[derive(Clone, PartialEq)]
    pub struct Kaart {
        kaart_kleur: KaartKleur,
        kaart_waarde: KaartWaarde,
    }

    impl Kaart {
        fn new() -> Self {
            Kaart {
                kaart_waarde: rand::random(),
                kaart_kleur: rand::random(),
            }
        }

        pub fn get_waarde(&self) -> u8 {
            match &self.kaart_waarde {
                KaartWaarde::TWEE => 2,
                KaartWaarde::DRIE => 3,
                KaartWaarde::VIER => 4,
                KaartWaarde::VIJF => 5,
                KaartWaarde::ZES => 6,
                KaartWaarde::ZEVEN => 7,
                KaartWaarde::ACHT => 8,
                KaartWaarde::NEGEN => 9,
                KaartWaarde::TIEN => 10,
                KaartWaarde::BOER => 10,
                KaartWaarde::DAME => 10,
                KaartWaarde::HEER => 10,
                KaartWaarde::AAS => 11,
            }
        }
    }

    impl StokKaarten {
        pub fn new() -> Self {
            StokKaarten {
                getrokken: Vec::new(),
                kaart_aantal: 0,
            }
        }
        pub fn trek_kaart(&mut self) -> Kaart {
            let mut new_card = false;
            let mut kaart: Kaart = Kaart::new();
            while !new_card {
                kaart = Kaart::new();
                if !self.getrokken.contains(&kaart) {
                    let temp_kaart = kaart.clone();
                    self.getrokken.push(temp_kaart);
                    new_card = true;
                }
            }
            kaart
        }
    }
}
