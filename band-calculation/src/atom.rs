/// Represents the atomic element.
enum AtomicElement {
    Unknown,
    Silicon,
    Arsenic,
    Indium,
}

/// Represents the ion type of atom.
enum IonType {
    Neutral,
    Cation,
    Anion,
}

///Represents the bonding state of atom.
enum BondingState {
    Dangling,
    Covalent,
}

/// Represents the space vector.
struct SpaceVector {
    /// The x component of a vector.
    x: f64,
    /// The y component of a vector.
    y: f64,
    /// The z component of a vector.
    z: f64,
}

const COORDINATION_NUMBER: usize = 4;

/**
 * Defines an atom module.
 */
struct Atom {
    /// position of Atom [nm].
    position: SpaceVector,
    /// ion type.
    ion: IonType,
    /// Atomic element.
    element: AtomicElement,
    /// bonding state List.
    bonding_state: [BondingState; COORDINATION_NUMBER],
}
