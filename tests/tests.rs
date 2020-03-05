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
