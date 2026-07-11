#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shipment {
    subtotal_cents: u32,
    weight_kg: u32,
    distance_km: u32,
}

impl Shipment {
    pub fn new(subtotal_cents: u32, weight_kg: u32, distance_km: u32) -> Self {
        Self {
            subtotal_cents,
            weight_kg,
            distance_km,
        }
    }

    pub fn subtotal_cents(&self) -> u32 {
        self.subtotal_cents
    }

    pub fn weight_kg(&self) -> u32 {
        self.weight_kg
    }

    pub fn distance_km(&self) -> u32 {
        self.distance_km
    }
}

pub trait ShippingStrategy {
    fn quote_cents(&self, shipment: &Shipment) -> u32;
}

#[derive(Debug, Clone, Copy)]
pub struct FlatRateShipping {
    rate_cents: u32,
}

impl FlatRateShipping {
    pub fn new(rate_cents: u32) -> Self {
        Self { rate_cents }
    }
}

impl ShippingStrategy for FlatRateShipping {
    fn quote_cents(&self, _shipment: &Shipment) -> u32 {
        self.rate_cents
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DistanceBasedShipping {
    base_cents: u32,
    cents_per_km: u32,
    cents_per_kg: u32,
}

impl DistanceBasedShipping {
    pub fn new(base_cents: u32, cents_per_km: u32, cents_per_kg: u32) -> Self {
        Self {
            base_cents,
            cents_per_km,
            cents_per_kg,
        }
    }
}

impl ShippingStrategy for DistanceBasedShipping {
    fn quote_cents(&self, shipment: &Shipment) -> u32 {
        self.base_cents
            + shipment.distance_km() * self.cents_per_km
            + shipment.weight_kg() * self.cents_per_kg
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FreeShippingOverSubtotal {
    minimum_subtotal_cents: u32,
    fallback_rate_cents: u32,
}

impl FreeShippingOverSubtotal {
    pub fn new(minimum_subtotal_cents: u32, fallback_rate_cents: u32) -> Self {
        Self {
            minimum_subtotal_cents,
            fallback_rate_cents,
        }
    }
}

impl ShippingStrategy for FreeShippingOverSubtotal {
    fn quote_cents(&self, shipment: &Shipment) -> u32 {
        if shipment.subtotal_cents() >= self.minimum_subtotal_cents {
            0
        } else {
            self.fallback_rate_cents
        }
    }
}

#[derive(Debug, Clone)]
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
    pub fn new(strategy: S) -> Self {
        Self { strategy }
    }

    pub fn quote_cents(&self, shipment: &Shipment) -> u32 {
        self.strategy.quote_cents(shipment)
    }
}
