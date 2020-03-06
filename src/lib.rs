/*
 *  Hinafont - ASCII font arts library
 *  Copyright (C) 2020  Chun-Kwong Wong
 *  chunkwong.wong@gmail.com
 *  https://github.com/shinkou/hinafont
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
/* Hinafont
┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌┬─┐┌─┐ ┌─┐┌┬─┐┌─┐ ┌──┐
│. ││ .││┌─┘│┬┐││ ─┤│ ─┤│┌─┤│  ││ │ │ ││ ┌┘│ │ │  │
│  ││ .││└─┐│┴┘││ ─┤│┌─┘│└ ││  ││ │┌┘ ││ └┐│ └┐││││
└┴─┘└──┘└──┘└──┘└──┘└┘  └──┘└┴─┘└─┘└──┘└┴─┘└──┘└┴┴┘
┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐┌┐┌┐┌┬─┐┌┬┬┐┌┬─┐┌┬─┐┌──┐
│  ││. ││ .││┌┐││. ││ ─┤└┐┌┘││││││ │││││└┐┌┘││ │├─ │
││ ││  ││┌─┘│└┘┤│ ─┤├─ │ ││ │└┘││  ││  │┌┘└┐└┐┌┘│ ─┤
└┴─┘└──┘└┘  └──┘└┴─┘└──┘ └┘ └──┘└──┘└──┘└┴─┘ └┘ └──┘
┌──┐┌─┐ ┌──┐┌──┐┌─┬┐┌──┐┌──┐┌──┐┌──┐┌──┐        
│┌┐│└┐│ └┐ │├─ ││ │││  ┤│ ─┤└─┐││. ││. │        
│└┘│┌┘└┐┌┘ ┤├─ │└┐ │├─ ││ .│  │││. │├─ │┌┐  ┌──┐
└──┘└──┘└──┘└──┘ └─┘└──┘└──┘  └┘└──┘└──┘└┘  └──┘
 ┌┐     ┌┐┌┐    ┌──┐┌┐  
┌┘└┐┌──┐└┼┼┘  ╱ └──┘└┘╱ 
└┐┌┘└──┘┌┼┼┐ ╱  ┌──┐ ╱┌┐
 └┘     └┘└┘    └──┘  └┘
┌┐  ┌┐  ┌─┐  ┌─┐┌──┐ ┌┐ ┌┐┌┐
└┘  ││  │┌┘  └┐│└┬─│ └┘ └┘└┘
┌┐  ├┤  │└┐  ┌┘│ ├┬┘        
└┘  └┘  └─┘  └─┘ └┘         
*/
pub fn conv(s: &str) -> String
{
	let tps: Vec<(&str, &str, &str, &str)> = s.to_uppercase().chars()
		.map(|c| to4x4(c)).collect();
	let mut s0: String = String::new();
	let mut s1: String = String::new();
	let mut s2: String = String::new();
	let mut s3: String = String::new();
	for t in &tps
	{
		let (t0, t1, t2, t3) = t;
		s0.push_str(t0);
		s1.push_str(t1);
		s2.push_str(t2);
		s3.push_str(t3);
	}
	let out = String::from(format!("{}\n{}\n{}\n{}\n", s0, s1, s2, s3));
	out
}

fn to4x4(c: char) -> (&'static str, &'static str, &'static str, &'static str)
{
	match c
	{
		'0' => ("┌──┐", "│┌┐│", "│└┘│", "└──┘")
		, '1' => ("┌─┐ ", "└┐│ ", "┌┘└┐", "└──┘")
		, '2' => ("┌──┐", "└┐ │", "┌┘ ┤", "└──┘")
		, '3' => ("┌──┐", "├─ │", "├─ │", "└──┘")
		, '4' => ("┌─┬┐", "│ ││", "└┐ │", " └─┘")
		, '5' => ("┌──┐", "│  ┤", "├─ │", "└──┘")
		, '6' => ("┌──┐", "│ ─┤", "│ .│", "└──┘")
		, '7' => ("┌──┐", "└─┐│", "  ││", "  └┘")
		, '8' => ("┌──┐", "│. │", "│. │", "└──┘")
		, '9' => ("┌──┐", "│. │", "├─ │", "└──┘")
		, 'A' => ("┌──┐", "│. │", "│  │", "└┴─┘")
		, 'B' => ("┌──┐", "│ .│", "│ .│", "└──┘")
		, 'C' => ("┌──┐", "│┌─┘", "│└─┐", "└──┘")
		, 'D' => ("┌──┐", "│┬┐│", "│┴┘│", "└──┘")
		, 'E' => ("┌──┐", "│ ─┤", "│ ─┤", "└──┘")
		, 'F' => ("┌──┐", "│ ─┤", "│┌─┘", "└┘  ")
		, 'G' => ("┌──┐", "│┌─┤", "│└ │", "└──┘")
		, 'H' => ("┌┬─┐", "│  │", "│  │", "└┴─┘")
		, 'I' => ("┌─┐", "│ │", "│ │", "└─┘")
		, 'J' => (" ┌─┐", " │ │", "┌┘ │", "└──┘")
		, 'K' => ("┌┬─┐", "│ ┌┘", "│ └┐", "└┴─┘")
		, 'L' => ("┌─┐ ", "│ │ ", "│ └┐", "└──┘")
		, 'M' => ("┌──┐", "│  │", "││││", "└┴┴┘")
		, 'N' => ("┌──┐", "│  │", "││ │", "└┴─┘")
		, 'O' => ("┌──┐", "│. │", "│  │", "└──┘")
		, 'P' => ("┌──┐", "│ .│", "│┌─┘", "└┘  ")
		, 'Q' => ("┌──┐", "│┌┐│", "│└┘┤", "└──┘")
		, 'R' => ("┌──┐", "│. │", "│ ─┤", "└┴─┘")
		, 'S' => ("┌──┐", "│ ─┤", "├─ │", "└──┘")
		, 'T' => ("┌──┐", "└┐┌┘", " ││ ", " └┘ ")
		, 'U' => ("┌┐┌┐", "││││", "│└┘│", "└──┘")
		, 'V' => ("┌┬─┐", "││ │", "│  │", "└──┘")
		, 'W' => ("┌┬┬┐", "││││", "│  │", "└──┘")
		, 'X' => ("┌┬─┐", "└┐┌┘", "┌┘└┐", "└┴─┘")
		, 'Y' => ("┌┬─┐", "││ │", "└┐┌┘", " └┘ ")
		, 'Z' => ("┌──┐", "├─ │", "│ ─┤", "└──┘")
		, '.' => ("    ", "    ", "┌┐  ", "└┘  ")
		, '_' => ("    ", "    ", "┌──┐", "└──┘")
		, '+' => (" ┌┐ ", "┌┘└┐", "└┐┌┘", " └┘ ")
		, '-' => ("    ", "┌──┐", "└──┘", "    ")
		, '*' => ("┌┐┌┐", "└┼┼┘", "┌┼┼┐", "└┘└┘")
		, '/' => ("    ", "  ╱ ", " ╱  ", "    ")
		, '=' => ("┌──┐", "└──┘", "┌──┐", "└──┘")
		, '%' => ("┌┐  ", "└┘╱ ", " ╱┌┐", "  └┘")
		, ':' => ("┌┐  ", "└┘  ", "┌┐  ", "└┘  ")
		, '!' => ("┌┐  ", "││  ", "├┤  ", "└┘  ")
		, '(' => ("┌─┐ ", "│┌┘ ", "│└┐ ", "└─┘ ")
		, ')' => (" ┌─┐", " └┐│", " ┌┘│", " └─┘")
		, '[' => ("┌─┐ ", "│┌┘ ", "│└┐ ", "└─┘ ")
		, ']' => (" ┌─┐", " └┐│", " ┌┘│", " └─┘")
		, '{' => ("┌─┐ ", "│┌┘ ", "│└┐ ", "└─┘ ")
		, '}' => (" ┌─┐", " └┐│", " ┌┘│", " └─┘")
		, '?' => ("┌──┐", "└┬─│", " ├┬┘", " └┘ ")
		, '\'' => (" ┌┐ ", " └┘ ", "    ", "    ")
		, '"' => ("┌┐┌┐", "└┘└┘", "    ", "    ")
		, ' ' => ("    ", "    ", "    ", "    ")
		, _ => ("", "", "", "")
	}
}
