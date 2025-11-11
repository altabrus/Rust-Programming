//! daily_homework_4 — RPG monetary system demo using structs, enums, and arrays
//!
//! Scenario: You work for Golden Dog Game Company. Design an in-game monetary
//! system with coins of different denominations, materials, and sizes. This
//! demo creates several coins, stores them in an array, and prints values and
//! totals.
//!
//! Notes:
//! - Base unit is "copper pieces" (cp). Other denominations convert to cp.
//! - We show how to use arrays (`[T; N]`) and slices (`&[T]`).

use std::fmt;

/// Denomination
///
/// Purpose: Represents coin denominations and their values in base units (copper pieces).
/// Values (in cp): Copper=1, Silver=10, Electrum=50, Gold=100, Platinum=1000
/// Type: `enum Denomination`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Denomination {
    Copper,
    Silver,
    Electrum,
    Gold,
    Platinum,
}

impl Denomination {
    /// value_in_cp
    ///
    /// Purpose: Get the denomination's worth in copper pieces (cp).
    /// Parameters: `self`
    /// Returns: `u32` value in cp
    /// Type: `fn value_in_cp(self) -> u32`
    fn value_in_cp(self) -> u32 {
        match self {
            Denomination::Copper => 1,
            Denomination::Silver => 10,
            Denomination::Electrum => 50,
            Denomination::Gold => 100,
            Denomination::Platinum => 1000,
        }
    }

    /// display_code
    ///
    /// Purpose: Short code string for the denomination (for concise printing).
    /// Returns: `&'static str`
    fn display_code(self) -> &'static str {
        match self {
            Denomination::Copper => "cp",
            Denomination::Silver => "sp",
            Denomination::Electrum => "ep",
            Denomination::Gold => "gp",
            Denomination::Platinum => "pp",
        }
    }
}

/// Material
///
/// Purpose: Represents the coin's physical material; impacts flavor only here.
/// Type: `enum Material`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
    Copper,
    Silver,
    Electrum,
    Gold,
    Platinum,
    Mithril,
    Obsidian,
}

/// Mint
///
/// Purpose: Location where coin was minted (flavor/world-building).
/// Type: `enum Mint`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mint {
    Capital,
    Coastal,
    Mountain,
    Desert,
}

/// Coin
///
/// Purpose: Represents a coin with denomination, material, size, year, and mint.
/// Type: `struct Coin`
#[derive(Debug, Clone, Copy, PartialEq)]
struct Coin {
    denom: Denomination,
    material: Material,
    diameter_mm: u16,
    thickness_mm: u16,
    year: u16,
    mint: Mint,
}

impl Coin {
    /// value_in_cp
    ///
    /// Purpose: Monetary value of this coin in copper pieces (cp).
    /// Parameters: `&self`
    /// Returns: `u32`
    /// Type: `fn value_in_cp(&self) -> u32`
    fn value_in_cp(&self) -> u32 {
        self.denom.value_in_cp()
    }
}

impl fmt::Display for Coin {
    /// Pretty print a coin as: "Gold (gp), Gold, 25×2 mm, Year 1023, Mint: Capital, 100 cp"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} ({}) — {:?}, {}×{} mm, Year {}, Mint: {:?}, {} cp",
            self.denom,
            self.denom.display_code(),
            self.material,
            self.diameter_mm,
            self.thickness_mm,
            self.year,
            self.mint,
            self.value_in_cp()
        )
    }
}

/// total_value_in_cp
///
/// Purpose: Sum the values of all coins in an array/slice in copper pieces.
/// Parameters: `coins: &[Coin]` — array or slice of coins
/// Returns: `u32` — total value in cp
/// Type: `fn total_value_in_cp(coins: &[Coin]) -> u32`
fn total_value_in_cp(coins: &[Coin]) -> u32 {
    coins.iter().map(|c| c.value_in_cp()).sum()
}

/// value_breakdown
///
/// Purpose: Convert a total copper amount into pp/gp/sp/cp for nicer display.
/// Parameters: `cp: u32`
/// Returns: `(pp, gp, sp, cp)`
/// Type: `fn value_breakdown(cp: u32) -> (u32, u32, u32, u32)`
fn value_breakdown(mut cp: u32) -> (u32, u32, u32, u32) {
    let pp = cp / 1000;
    cp %= 1000;
    let gp = cp / 100;
    cp %= 100;
    let sp = cp / 10;
    cp %= 10;
    (pp, gp, sp, cp)
}

fn main() {
    // --- Create an array of coins (fixed-size array demonstrates "arrays") ---
    // Each coin: denomination, material, size, year, mint.
    let wallet: [Coin; 6] = [
        Coin { denom: Denomination::Gold,     material: Material::Gold,     diameter_mm: 25, thickness_mm: 2, year: 1023, mint: Mint::Capital },
        Coin { denom: Denomination::Silver,   material: Material::Silver,   diameter_mm: 22, thickness_mm: 2, year: 1023, mint: Mint::Coastal },
        Coin { denom: Denomination::Copper,   material: Material::Copper,   diameter_mm: 21, thickness_mm: 2, year: 1022, mint: Mint::Mountain },
        Coin { denom: Denomination::Platinum, material: Material::Platinum, diameter_mm: 27, thickness_mm: 2, year: 1024, mint: Mint::Capital },
        Coin { denom: Denomination::Electrum, material: Material::Electrum, diameter_mm: 24, thickness_mm: 2, year: 1021, mint: Mint::Desert },
        Coin { denom: Denomination::Gold,     material: Material::Mithril,  diameter_mm: 25, thickness_mm: 2, year: 1020, mint: Mint::Mountain },
    ];

    println!("--- Golden Dog RPG Coin Demonstration ---\n");
    for (i, coin) in wallet.iter().enumerate() {
        println!("Coin #{i}: {coin}");
    }

    let total_cp = total_value_in_cp(&wallet);
    let (pp, gp, sp, cp) = value_breakdown(total_cp);

    println!("\nTotal value: {total_cp} cp  →  {pp} pp, {gp} gp, {sp} sp, {cp} cp");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_denom_values() {
        assert_eq!(Denomination::Copper.value_in_cp(), 1);
        assert_eq!(Denomination::Silver.value_in_cp(), 10);
        assert_eq!(Denomination::Electrum.value_in_cp(), 50);
        assert_eq!(Denomination::Gold.value_in_cp(), 100);
        assert_eq!(Denomination::Platinum.value_in_cp(), 1000);
    }

    #[test]
    fn test_total_and_breakdown() {
        let purse = [
            Coin { denom: Denomination::Gold,     material: Material::Gold,     diameter_mm: 25, thickness_mm: 2, year: 1023, mint: Mint::Capital },
            Coin { denom: Denomination::Silver,   material: Material::Silver,   diameter_mm: 22, thickness_mm: 2, year: 1023, mint: Mint::Coastal },
            Coin { denom: Denomination::Copper,   material: Material::Copper,   diameter_mm: 21, thickness_mm: 2, year: 1022, mint: Mint::Mountain },
            Coin { denom: Denomination::Platinum, material: Material::Platinum, diameter_mm: 27, thickness_mm: 2, year: 1024, mint: Mint::Capital },
            Coin { denom: Denomination::Electrum, material: Material::Electrum, diameter_mm: 24, thickness_mm: 2, year: 1021, mint: Mint::Desert },
        ];
        let total_cp = total_value_in_cp(&purse); // 100 + 10 + 1 + 1000 + 50 = 1161
        assert_eq!(total_cp, 1161);
        assert_eq!(value_breakdown(total_cp), (1, 1, 6, 1)); // 1pp, 1gp, 6sp, 1cp
    }
}
