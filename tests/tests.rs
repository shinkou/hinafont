/*
 *  Hinafont - ASCII font arts library
 *  Copyright (C) 2020  Chun-Kwong Wong
 *  chunkwong.wong@gmail.com
 *  https://github.com/shinkou/hinagen
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
extern crate hinafont;

#[test]
fn it_works()
{
	let cand = hinafont::conv("hinafont test");
	let ctrl: &str = "\
┌┬─┐┌─┐┌──┐┌──┐┌──┐┌──┐┌──┐┌──┐    ┌──┐┌──┐┌──┐┌──┐\n\
│  ││ ││  ││. ││ ─┤│. ││  │└┐┌┘    └┐┌┘│ ─┤│ ─┤└┐┌┘\n\
│  ││ │││ ││  ││┌─┘│  │││ │ ││      ││ │ ─┤├─ │ ││ \n\
└┴─┘└─┘└┴─┘└┴─┘└┘  └──┘└┴─┘ └┘      └┘ └──┘└──┘ └┘ \n";
	println!("{}", ctrl);
	println!("{}", cand);
	assert_eq!(cand, ctrl);
}
