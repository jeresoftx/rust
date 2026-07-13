#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Tipo publico `Shipment` usado por el ejemplo para expresar el dominio del patron.
pub struct Shipment {
    subtotal_cents: u32,
    weight_kg: u32,
    distance_km: u32,
}

impl Shipment {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(subtotal_cents: u32, weight_kg: u32, distance_km: u32) -> Self {
        Self {
            subtotal_cents,
            weight_kg,
            distance_km,
        }
    }

    /// Modela la operacion `subtotal cents` dentro del ejemplo del patron.
    pub fn subtotal_cents(&self) -> u32 {
        self.subtotal_cents
    }

    /// Modela la operacion `weight kg` dentro del ejemplo del patron.
    pub fn weight_kg(&self) -> u32 {
        self.weight_kg
    }

    /// Modela la operacion `distance km` dentro del ejemplo del patron.
    pub fn distance_km(&self) -> u32 {
        self.distance_km
    }
}

/// Contrato publico `ShippingStrategy` que desacopla las piezas del ejemplo.
pub trait ShippingStrategy {
    /// Operacion `quote cents` definida por la abstraccion del ejemplo.
    fn quote_cents(&self, shipment: &Shipment) -> u32;
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `FlatRateShipping` usado por el ejemplo para expresar el dominio del patron.
pub struct FlatRateShipping {
    rate_cents: u32,
}

impl FlatRateShipping {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(rate_cents: u32) -> Self {
        Self { rate_cents }
    }
}

impl ShippingStrategy for FlatRateShipping {
    /// Operacion `quote cents` definida por la abstraccion del ejemplo.
    fn quote_cents(&self, _shipment: &Shipment) -> u32 {
        self.rate_cents
    }
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `DistanceBasedShipping` usado por el ejemplo para expresar el dominio del patron.
pub struct DistanceBasedShipping {
    base_cents: u32,
    cents_per_km: u32,
    cents_per_kg: u32,
}

impl DistanceBasedShipping {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(base_cents: u32, cents_per_km: u32, cents_per_kg: u32) -> Self {
        Self {
            base_cents,
            cents_per_km,
            cents_per_kg,
        }
    }
}

impl ShippingStrategy for DistanceBasedShipping {
    /// Operacion `quote cents` definida por la abstraccion del ejemplo.
    fn quote_cents(&self, shipment: &Shipment) -> u32 {
        self.base_cents
            + shipment.distance_km() * self.cents_per_km
            + shipment.weight_kg() * self.cents_per_kg
    }
}

#[derive(Debug, Clone, Copy)]
/// Tipo publico `FreeShippingOverSubtotal` usado por el ejemplo para expresar el dominio del patron.
pub struct FreeShippingOverSubtotal {
    minimum_subtotal_cents: u32,
    fallback_rate_cents: u32,
}

impl FreeShippingOverSubtotal {
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(minimum_subtotal_cents: u32, fallback_rate_cents: u32) -> Self {
        Self {
            minimum_subtotal_cents,
            fallback_rate_cents,
        }
    }
}

impl ShippingStrategy for FreeShippingOverSubtotal {
    /// Operacion `quote cents` definida por la abstraccion del ejemplo.
    fn quote_cents(&self, shipment: &Shipment) -> u32 {
        if shipment.subtotal_cents() >= self.minimum_subtotal_cents {
            0
        } else {
            self.fallback_rate_cents
        }
    }
}

#[derive(Debug, Clone)]
/// Tipo publico `ShippingCalculator` usado por el ejemplo para expresar el dominio del patron.
pub struct ShippingCalculator<S>
where
    S: ShippingStrategy,
{
    strategy: S,
}

impl<S> ShippingCalculator<S>
where
    S: ShippingStrategy,
{
    /// Crea una instancia valida para el ejemplo del patron.
    pub fn new(strategy: S) -> Self {
        Self { strategy }
    }

    /// Modela la operacion `quote cents` dentro del ejemplo del patron.
    pub fn quote_cents(&self, shipment: &Shipment) -> u32 {
        self.strategy.quote_cents(shipment)
    }
}
