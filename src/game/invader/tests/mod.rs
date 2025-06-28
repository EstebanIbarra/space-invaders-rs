use super::*;

#[test]
fn test_new_invader() {
    let mut invader = Invader::new((0, 2), ShieldType::None, InvaderType::Alpha, 1);
    let mut expected = Invader {
        position: (0, 2),
        r#type: InvaderType::Alpha,
        shield: Shield::new((0, 0), ShieldType::None, Owner::Invader),
        health: 1,
        alive: true,
        sprite: ALPHA,
    };
    assert_eq!(invader, expected);

    invader = Invader::new((0, 0), ShieldType::Single, InvaderType::Beta, 1);
    expected = Invader {
        position: (0, 2),
        r#type: InvaderType::Beta,
        shield: Shield::new((0, 0), ShieldType::Single, Owner::Invader),
        health: 2,
        alive: true,
        sprite: BETA,
    };
    assert_eq!(invader, expected);
}
