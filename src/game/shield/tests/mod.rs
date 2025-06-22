use super::*;

#[test]
fn test_new_shield() {
    let mut shield = Shield::new((0, 0), ShieldType::None, Owner::Player);
    let mut expected = Shield {
        position: (0, 0),
        r#type: ShieldType::None,
        owner: Owner::Player,
        sprite: NO_SPRITE,
    };
    assert_eq!(shield, expected);

    shield = Shield::new((0, 0), ShieldType::Single, Owner::Player);
    expected = Shield {
        position: (0, 0),
        r#type: ShieldType::Single,
        owner: Owner::Player,
        sprite: P_S_SHIELD,
    };
    assert_eq!(shield, expected);

    shield = Shield::new((0, 0), ShieldType::Double, Owner::Player);
    expected = Shield {
        position: (0, 0),
        r#type: ShieldType::Double,
        owner: Owner::Player,
        sprite: P_D_SHIELD,
    };
    assert_eq!(shield, expected);

    shield = Shield::new((0, 0), ShieldType::Triple, Owner::Player);
    expected = Shield {
        position: (0, 0),
        r#type: ShieldType::Triple,
        owner: Owner::Player,
        sprite: P_T_SHIELD,
    };
    assert_eq!(shield, expected);

    shield = Shield::new((0, 0), ShieldType::Quad, Owner::Player);
    expected = Shield {
        position: (0, 0),
        r#type: ShieldType::Quad,
        owner: Owner::Player,
        sprite: P_Q_SHIELD,
    };
    assert_eq!(shield, expected);

    shield = Shield::new((0, 0), ShieldType::None, Owner::Invader);
    expected = Shield {
        position: (0, 0),
        r#type: ShieldType::None,
        owner: Owner::Invader,
        sprite: NO_SPRITE,
    };
    assert_eq!(shield, expected);

    shield = Shield::new((0, 0), ShieldType::Single, Owner::Invader);
    expected = Shield {
        position: (0, 0),
        r#type: ShieldType::Single,
        owner: Owner::Invader,
        sprite: I_S_SHIELD,
    };
    assert_eq!(shield, expected);

    shield = Shield::new((0, 0), ShieldType::Double, Owner::Invader);
    expected = Shield {
        position: (0, 0),
        r#type: ShieldType::Double,
        owner: Owner::Invader,
        sprite: I_D_SHIELD,
    };
    assert_eq!(shield, expected);

    shield = Shield::new((0, 0), ShieldType::Triple, Owner::Invader);
    expected = Shield {
        position: (0, 0),
        r#type: ShieldType::Triple,
        owner: Owner::Invader,
        sprite: I_T_SHIELD,
    };
    assert_eq!(shield, expected);

    shield = Shield::new((0, 0), ShieldType::Quad, Owner::Invader);
    expected = Shield {
        position: (0, 0),
        r#type: ShieldType::Quad,
        owner: Owner::Invader,
        sprite: I_Q_SHIELD,
    };
    assert_eq!(shield, expected);
}

#[test]
fn test_upgrade_shield() {
    let mut shield = Shield::new((0, 0), ShieldType::None, Owner::Player);
    shield.upgrade();
    let mut expected = Shield::new((0, 0), ShieldType::Single, Owner::Player);
    assert_eq!(shield, expected);

    shield.upgrade();
    expected = Shield::new((0, 0), ShieldType::Double, Owner::Player);
    assert_eq!(shield, expected);

    shield.upgrade();
    expected = Shield::new((0, 0), ShieldType::Triple, Owner::Player);
    assert_eq!(shield, expected);

    shield.upgrade();
    expected = Shield::new((0, 0), ShieldType::Quad, Owner::Player);
    assert_eq!(shield, expected);

    shield.upgrade();
    assert_eq!(shield, expected);
}

#[test]
fn test_downgrade_shield() {
    let mut shield = Shield::new((0, 0), ShieldType::Quad, Owner::Player);
    shield.downgrade();
    let mut expected = Shield::new((0, 0), ShieldType::Triple, Owner::Player);
    assert_eq!(shield, expected);

    shield.downgrade();
    expected = Shield::new((0, 0), ShieldType::Double, Owner::Player);
    assert_eq!(shield, expected);

    shield.downgrade();
    expected = Shield::new((0, 0), ShieldType::Single, Owner::Player);
    assert_eq!(shield, expected);

    shield.downgrade();
    expected = Shield::new((0, 0), ShieldType::None, Owner::Player);
    assert_eq!(shield, expected);

    shield.downgrade();
    assert_eq!(shield, expected);
}
