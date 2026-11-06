//! Paperdoll / equipment layer metadata: body kinds, wearable layers and the client's draw order
//! (taken from the `<wearables><draworder>` blocks of MainMisc.uop).

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BodyKind {
    HumanMale,
    HumanFemale,
    ElfMale,
    ElfFemale,
    GargoyleMale,
    GargoyleFemale,
}

impl BodyKind {
    pub const ALL: [BodyKind; 6] = [
        Self::HumanMale,
        Self::HumanFemale,
        Self::ElfMale,
        Self::ElfFemale,
        Self::GargoyleMale,
        Self::GargoyleFemale,
    ];
    pub fn label(&self) -> &'static str {
        match self {
            Self::HumanMale => "Humano ♂",
            Self::HumanFemale => "Humano ♀",
            Self::ElfMale => "Elfo ♂",
            Self::ElfFemale => "Elfo ♀",
            Self::GargoyleMale => "Gárgula ♂",
            Self::GargoyleFemale => "Gárgula ♀",
        }
    }
    /// Animation body id (also the "body type" key of Equipconv.def).
    pub fn anim_body(&self) -> u32 {
        match self {
            Self::HumanMale => 400,
            Self::HumanFemale => 401,
            Self::ElfMale => 605,
            Self::ElfFemale => 606,
            Self::GargoyleMale => 666,
            Self::GargoyleFemale => 667,
        }
    }
    pub fn female(&self) -> bool {
        matches!(self, Self::HumanFemale | Self::ElfFemale | Self::GargoyleFemale)
    }
    /// Gump of the bare paperdoll body.
    pub fn body_gump(&self) -> u32 {
        match self {
            Self::HumanMale => 12,
            Self::HumanFemale => 13,
            Self::ElfMale => 14,
            Self::ElfFemale => 15,
            Self::GargoyleMale => 665,
            Self::GargoyleFemale => 666,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LayerInfo {
    pub id: u8,
    pub name: &'static str,
    /// draw order while animating (lower = behind)
    pub anim_order: i32,
    /// draw order in the paperdoll gump (lower = behind)
    pub doll_order: i32,
}

const fn l(id: u8, name: &'static str, anim_order: i32, doll_order: i32) -> LayerInfo {
    LayerInfo { id, name, anim_order, doll_order }
}

/// Wearable layers (UO layer numbers) with the draw orders of the client.
pub const LAYERS: [LayerInfo; 22] = [
    l(1, "Mão direita (1H)", 23000, 24000),
    l(2, "Mão esquerda (2H / escudo)", 24000, 23000),
    l(3, "Sapatos", 5000, 5000),
    l(4, "Calças", 6000, 6000),
    l(5, "Camisa", 7000, 7000),
    l(6, "Elmo / chapéu", 21000, 21000),
    l(7, "Luvas", 16500, 16500),
    l(8, "Anel", 2000, 2000),
    l(9, "Talismã", 9000, 9000),
    l(10, "Pescoço", 10000, 18100),
    l(11, "Cabelo", 20500, 20000),
    l(12, "Cinto", 18500, 18500),
    l(13, "Torso interno", 12000, 12000),
    l(14, "Bracelete", 16750, 16750),
    l(16, "Barba", 20750, 20750),
    l(17, "Torso médio", 15000, 15000),
    l(18, "Brincos", 3000, 3000),
    l(19, "Braços", 16000, 16000),
    l(20, "Capa", 20000, 500),
    l(21, "Mochila", 22000, 22000),
    l(22, "Túnica externa / robe", 18000, 18000),
    l(24, "Pernas internas", 19000, 19000),
];

pub fn layer_info(id: u8) -> Option<&'static LayerInfo> {
    LAYERS.iter().find(|l| l.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cape_is_behind_in_paperdoll_but_in_front_when_animating() {
        let cape = layer_info(20).unwrap();
        let shirt = layer_info(5).unwrap();
        assert!(cape.doll_order < shirt.doll_order);
        assert!(cape.anim_order > shirt.anim_order);
    }

    #[test]
    fn body_kinds() {
        assert_eq!(BodyKind::HumanFemale.anim_body(), 401);
        assert!(BodyKind::ElfFemale.female());
        assert_eq!(BodyKind::HumanMale.body_gump(), 12);
    }
}
