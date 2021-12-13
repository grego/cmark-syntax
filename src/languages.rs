// This file is part of cmark-syntax. This program comes with ABSOLUTELY NO WARRANTY;
// This is free software, and you are welcome to redistribute it under the
// conditions of the GNU General Public License version 3.0.
//
// You should have received a copy of the GNU General Public License
// along with cmark-syntax.  If not, see <http://www.gnu.org/licenses/>
mod javascript;
mod rust;
mod toml;

pub use javascript::JavaScript;
pub use rust::Rust;
pub use toml::Toml;
