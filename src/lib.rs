#![no_std]
use no_std_compat::prelude::v1::*;

use keytokey::{
    handlers, debug_handlers,
    Event, EventStatus, KeyCode, Keyboard, ProcessKeys,
    USBKeyOut, 
    premade
};
// pins for matrix - see main.rs, otherwise the borrow checker has a fit :(

pub fn get_translation() -> Vec<u32> {
    use keytokey::KeyCode::*;
    vec![
        0, //one of the missing nes?
        //00000001
        V.into(),
        //00000002
        SColon.into(),
        //00000003
        F8.into(),
        //00000004
        K.into(),
        //00000005
        Space.into(),
        //00000006
        BSpace.into(),
        //00000007
        Q.into(),
        //00000008
        F2.into(),
        //9
        9,
        //0000000a
        C.into(),
        //0000000b
        L.into(),
        //c
        0xc,
        //0000000d
        J.into(),
        //0000000e
        LGui.into(),
        //0000000f
        Delete.into(), // which one is this?
        //00000010
        A.into(),
        //00000011
        F1.into(),
        //12
        0x12,
        //13
        D.into(),
        //14
        LBracket.into(),
        //15
        Equal.into(),
        //16
        Down.into(),
        //17
        Home.into(),
        //18
        PgUp.into(),
        //19
        0xF0100u32.into(), // te lowe left backslash key
        //1A
        Escape.into(),
        //1B
        0x1b,
        //0000001c
        E.into(),
        //0000001d
        Slash.into(), //label: slash
        //0000001e
        LBracket.into(), //top right, label bslash
        //0000001f
        Comma.into(),
        //0x20
        Enter.into(),
        //00000021
        LAlt.into(),
        //00000022
        Z.into(),
        //00000023
        Minus.into(),
        //24
        0x24,
        //00000025
        Right.into(),
        //00000026
        Dot.into(),
        //00000027
        Quote.into(),
        //00000028
        M.into(),
        //00000029
        End.into(),
        //0000002a
        LCtrl.into(),
        //0000002b
        X.into(),
        //2c
        Tab.into(),
        //2d
        0x2d,
        //0000002e
        Left.into(),
        //0000002f
        RBracket.into(),
        //00000030
        RShift.into(),
        //00000031
        Up.into(),
        //00000032
        RCtrl.into(),
        //00000033
        PgDown.into(),
        //00000034
        Grave.into(),
        //00000035
        LShift.into(),
        //36
        Copy.into(),//palm1
        //00000037
        Kb3.into(),
        //00000038
        Kb0.into(),
        //00000039
        F12.into(),
        //0000003a
        Kb8.into(),
        //0000003b
        Kb6.into(),
        //0000003c
        Kb5.into(),
        //0000003d
        Kb1.into(),
        //0000003e
        F6.into(),
        //3f
        Paste.into(), ///palm 2
        //00000040
        Kb4.into(),
        //00000041
        Kb9.into(),
        //00000042
        F11.into(),
        //00000043
        Kb7.into(),
        //00000044
        Y.into(),
        //00000045
        T.into(),
        //00000046
        Kb2.into(),
        //00000047
        F5.into(),
        //48,
        0, //palm3
        //00000049
        R.into(),
        //0000004a
        P.into(),
        //0000004b
        F10.into(),
        //0000004c
        I.into(),
        //0000004d
        H.into(),
        //0000004e
        G.into(),
        //0000004f
        W.into(),
        //00000050
        F3.into(),
        //51
        0x51,
        //00000052
        F.into(),
        //00000053
        O.into(),
        //00000054
        F9.into(),
        //00000055
        U.into(),
        //00000056
        N.into(),
        //00000057
        B.into(),
        //00000058
        S.into(),
        //00000059
        F4.into(),
    ]
    /*

    .into()
    01	KeyCode::.into()
    01	KeyCode::
            not working:  F7
            */
    //position -> keycode.
    //if too short, automatically extended with running number.
    // So you can start with an empty one, + key2key::debug_handlers::TranslationHelper
    // in the layout
}
/*
pub struct Debugger {
    s: String
}
impl Debugger {
    #[allow(dead_code)]
    fn new(s: String)->Debugger{
        Debugger{s}
    }
}

impl ProcessKeys<USBOut> for Debugger {
    fn process_keys(&mut self, events: &mut Vec<(Event, EventStatus)>, output: &mut USBOut) -> () {
        output.tx.writeln(&format!("{}, {:?}", self.s, events));
    }
}
*/

pub fn get_keytokey<'a, T: USBKeyOut>(output: T) -> Keyboard<'a, T> {
    let mut k = Keyboard::new(output);
    /*
    use handlers::LayerAction::RewriteTo as RT;
    let numpad_id = k.add_handler(Box::new(
        handlers::Layer::new(vec![
            (KeyCode::U, RT(KeyCode::Kb7.into())),
            (KeyCode::I, RT(KeyCode::Kb8.into())),
            (KeyCode::O, RT(KeyCode::Kb9.into())),
            (KeyCode::J, RT(KeyCode::Kb4.into())),
            (KeyCode::K, RT(KeyCode::Kb5.into())),
            (KeyCode::L, RT(KeyCode::Kb6.into())),
            (KeyCode::M, RT(KeyCode::Kb1.into())),
            (KeyCode::Comma, RT(KeyCode::Kb2.into())),
            (KeyCode::Dot, RT(KeyCode::Kb3.into())),
            (KeyCode::Up, RT(KeyCode::Kb0.into())),
            (KeyCode::Space, RT(KeyCode::Tab.into())),
        ])
    )
    );
    k.add_handler(premade::space_cadet_handler(KeyCode::F, numpad_id));
    */
    /*
    k.add_handler(Box::new(handlers::Leader::new(
        0xF0100u32,
        vec![
            (vec![KeyCode::Y], "Dr. Florian Finkernagel"),
            (vec![KeyCode::U], "f.finkernagel@coonabibba.de"),
            (vec![KeyCode::F], "finkernagel@imt.uni-marburg.de"),
        ],
        ""
    )));
    */


    let dvorak_id = k.add_handler(premade::dvorak());
    k.add_handler(premade::toggle_handler(KeyCode::F1, dvorak_id));
    k.output.state().enable_handler(dvorak_id);

    k.add_handler(premade::one_shot_shift());
    k.add_handler(premade::one_shot_ctrl());
    k.add_handler(premade::one_shot_alt());
    k.add_handler(premade::one_shot_gui());
    k.add_handler(Box::new(premade::CopyPaste{}));
    k.add_handler(Box::new(handlers::USBKeyboard::new()));
    k.add_handler(Box::new(debug_handlers::TranslationHelper {}));
    return k;
}

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests {
    #[macro_use]
    use std::dbg;
    use crate::get_keytokey;
    use keytokey::test_helpers::{KeyOutCatcher, check_output};
    use keytokey::KeyCode;
    #[test]
    fn test_brackets() {
        let mut keyboard = get_keytokey(KeyOutCatcher::new());
        keyboard.add_keypress(KeyCode::LBracket, 0);
        keyboard.handle_keys().unwrap();
        check_output(&keyboard, &[&[KeyCode::Slash]]);
        keyboard.add_keyrelease(KeyCode::LBracket, 0);
        keyboard.handle_keys().unwrap();
        keyboard.output.clear();

        keyboard.add_keypress(KeyCode::RBracket, 0);
        keyboard.handle_keys().unwrap();
        dbg!(&keyboard.output.reports);
        check_output(&keyboard, &[&[KeyCode::Equal]]);
        keyboard.add_keyrelease(KeyCode::RBracket, 0);
        keyboard.handle_keys().unwrap();
        keyboard.output.clear();


    }
}
