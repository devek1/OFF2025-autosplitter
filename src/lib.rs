use std::collections::{HashSet};
use asr::string::{ArrayWString};
use asr::{future::next_tick, game_engine::{
    unity::mono::{Module, UnityPointer, Version},
}, settings::Gui, timer::{self}, watcher::{Watcher, Pair}, Process};
use std::ops::Deref;
use asr::settings::gui::Title;
use asr::timer::TimerState;

asr::async_main!(stable);




#[derive(Gui)]
struct Settings {
    /// Auto-start on name confirmation
    #[default = true]
    autostart: bool,
    /// Auto-reset on name confirmation
    #[default = true]
    autoreset: bool,

    zone_0_splits : Title,
    /// Zone 0
    #[default = false]
    zone0 : bool,
    /// Sugar
    #[default = false]
    sugar : bool,

    zone_1_splits : Title,
    /// Obtain Add-On Alpha
    #[default = false]
    alpha_party_member : bool,
    /// Enter Mines
    #[default = false]
    enter_mines : bool,
    /// Mines
    #[default = false]
    mines : bool,
    /// Barn
    #[default = false]
    barn : bool,
    /// Enter Postal Service
    #[default = false]
    enter_postal_service : bool,
    /// Postal Service
    #[default = false]
    postal_service : bool,
    /// Alma First Half
    #[default = false]
    alma_first_half : bool,
    /// Alma Second Half
    #[default = false]
    alma_second_half : bool,
    /// Zone 1
    #[default = true]
    zone1 : bool,


    zone_2_splits : Title,
    /// Card Puzzle
    #[default = false]
    card_puzzle : bool,
    /// Japhet 1
    #[default = false]
    japhet1 : bool,
    /// Open the Zacharie Rollercoaster Photo
    #[default = false]
    zacharie_photo : bool,
    /// Park
    #[default = false]
    park : bool,
    /// Residential
    #[default = false]
    residential : bool,
    /// Enter Japhet
    #[default = false]
    enter_japhet : bool,
    /// Zone 2
    #[default = true]
    zone2 : bool,

    zone_3_splits : Title,
    /// Area 1
    #[default = false]
    area1 : bool,
    /// Area 2
    #[default = false]
    area2 : bool,
    /// Area 3
    #[default = false]
    area3 : bool,
    /// Elsen Fight
    #[default = false]
    critic_burnt: bool,
    /// Area 4
    #[default = false]
    area4 : bool,
    /// Enoch
    #[default = true]
    enoch : bool,

    the_room_splits : Title,
    /// Chapter 5
    #[default = false]
    chapter5 : bool,
    /// Chapter 4
    #[default = false]
    chapter4 : bool,
    /// Chapter 3
    #[default = false]
    chapter3 : bool,
    /// Exit The Room
    #[default = false]
    exit_the_room : bool,
    /// Chapter 2
    #[default = false]
    chapter2 : bool,
    /// Chapter 1
    #[default = false]
    chapter1 : bool,

    ending_splits : Title,
    /// Final Fight, Judge Ending (Adversaries purified)
    #[default = true]
    bad_batter : bool,
    /// Batter Ending (Turning the switch off)
    #[default = true]
    ending_switch : bool,

    pure_zone_splits : Title,
    /// Pure Zone 1
    #[default = false]
    pure_zone1 : bool,
    /// Pure Zone 2
    #[default = false]
    pure_zone2 : bool,
    /// Pure Zone 3
    #[default = false]
    pure_zone3 : bool,

    uber_boss_splits : Title,
    /// Source
    #[default = false]
    uber1_source : bool,
    /// Maldicion
    #[default = false]
    uber2_maldicion : bool,
    /// Psalmanazar and Herodotus
    #[default = false]
    uber3_psal_hero : bool,
    /// Justus
    #[default = false]
    uber4_justus : bool,
    /// Carnival
    #[default = false]
    uber5_carnival : bool,
    /// Cob
    #[default = false]
    uber6_cob : bool,

}

//Note: the split function doesn't check whether the split is enabled or whether it's already triggered, because that's checked earlier.
pub fn split(splits : &mut HashSet<String>, name : &str) {
    splits.insert(name.to_string());
    asr::print_message(name);
    timer::split();
}

pub fn check_map_split(map_state : Pair<i32>, splits : &mut HashSet<String>, setting : bool, name : &str, orig_map : i32, new_map : i32) {
    if setting && !splits.contains(name) {
        if (orig_map == -1 || orig_map == map_state.old) && new_map == map_state.current {
            split(splits, name);
        }
    }
}

pub fn check_event_exact_split(current_map: i32, current_event : i32, current_page : i32, current_line : Pair<i32>, splits : &mut HashSet<String>, setting : bool, name : &str, map_id : i32, event_id : i32, page_index : i32, line_number : i32) {
    if setting && !splits.contains(name) {
        if (map_id == -1 || map_id == current_map) && current_event == event_id && current_page == page_index && (current_line.old < line_number && current_line.current == line_number) {
            split(splits, name);
        }
    }
}
pub fn check_event_atleast_split(current_map: i32, current_event : i32, current_page : i32, current_line : Pair<i32>, splits : &mut HashSet<String>, setting : bool, name : &str, map_id : i32, event_id : i32, page_index : i32, line_number : i32) {
    if setting && !splits.contains(name) {
        if (map_id == -1 || map_id == current_map) && current_event == event_id && current_page == page_index && (current_line.old < line_number || (line_number == 0 && current_line.old != 0)) && current_line.current >= line_number {
            split(splits, name);
        }
    }
}
pub fn check_event_new_page(current_map: i32, current_event : i32, current_page : Pair<i32>, splits : &mut HashSet<String>, setting : bool, name : &str, map_id : i32, event_id : i32, page_index : i32) {
    if setting && !splits.contains(name) {
        if (map_id == -1 || map_id == current_map) && current_event == event_id && current_page.old != page_index && current_page.current == page_index {
            split(splits, name);
        }
    }
}
pub fn start_battle_in_map_split(current_map : i32, in_battle : Pair<bool>, splits : &mut HashSet<String>, setting : bool, name : &str, map_id : i32) {
    if setting && !splits.contains(name) {
        if map_id == current_map && !in_battle.old && in_battle.current {
            split(splits, name);
        }
    }
}
pub fn win_battle_in_map_split(current_map : i32, battle_result : Pair<i32>, in_battle : Pair<bool>, splits : &mut HashSet<String>, setting : bool, name : &str, map_id : i32) {
    if setting && !splits.contains(name) {
        if map_id == current_map && !in_battle.current && in_battle.old && battle_result.current == 5 {
            split(splits, name);
        }
    }
}





async fn main() {
    //LiveSplit-related objects
    let mut settings = Settings::register();
    let mut splits = HashSet::<String>::new();

    //UnityPointers
    let map_id_pointer = UnityPointer::<3>::new("FangamerRPG.FPGOverworldMode",0,&["instance","m_mapComp","mapID"],);
    let event_id_pointer = UnityPointer::<5>::new("FangamerRPG.FPGLogicManager",0,&["instance", "_currentInterpreter", "_state", "owner", "eventID"]);
    let logic_interpreter_pointer = UnityPointer::<4>::new("FangamerRPG.FPGLogicManager",0,&["instance", "_currentInterpreter", "ownerName", "0x14"]);
    let event_page_pointer = UnityPointer::<4>::new("FangamerRPG.FPGLogicManager",0,&["instance", "_currentInterpreter", "_state", "pageIndex"]);
    let event_line_pointer = UnityPointer::<3>::new("FangamerRPG.FPGLogicManager",0,&["instance", "_currentInterpreter", "currentLine"]);
    let in_battle_pointer = UnityPointer::<3>::new("FangamerRPG.FPGOverworldMode",0,&["instance", "battleManager", "inBattle"]);
    let battle_result_pointer = UnityPointer::<2>::new("OFFGame.Battle.BATMain",0,&["instance", "result"]);
    let adversaries_purified_pointer = UnityPointer::<4>::new("OFFGame.Battle.BATMain",0,&["instance", "youWon", "0x10", "0x47"]);
    //watchers to keep track of old and new states of values
    let mut map_watcher = Watcher::<i32>::new();
    let mut event_id_watcher = Watcher::<i32>::new();
    let mut event_page_watcher = Watcher::<i32>::new();
    let mut in_battle_watcher = Watcher::<bool>::new();
    let mut zacharie_photo_watcher = Watcher::<bool>::new();
    let mut battle_result_watcher = Watcher::<i32>::new();
    let mut adversaries_purified_watcher = Watcher::<bool>::new();
    let mut event_line_watcher = Watcher::<i32>::new();

    //Currently no use for SceneManager (for some reason the old autosplitter used the scene exclusively to detect if a battle was won, even though it could already detect the "Adversaries Purified" text that appears whenever a battle is won
    //let mut scene_watcher = Watcher::<String>::new();

    loop {

        let process = Process::wait_attach("OFF.exe").await;
        asr::print_message("Found potential OFF game process");

        process
            .until_closes(async {
                let module = Module::wait_attach(&process, Version::V3).await;
                asr::print_message("Found Mono module");
                let image = module.wait_get_default_image(&process).await;
                asr::print_message("Found Assembly image");
                //let scene_manager = SceneManager::wait_attach(&process).await;
                //asr::print_message("Found Scene Manager");



                loop {
                    settings.update();

                    let map_id = map_watcher.update_infallible(map_id_pointer.deref(&process, &module, &image).unwrap_or_else(|e| -1));
                    let event_id = event_id_watcher.update_infallible(event_id_pointer.deref(&process, &module, &image).unwrap_or_else(|e| -1));
                    let event_page = event_page_watcher.update_infallible(event_page_pointer.deref(&process, &module, &image).unwrap_or_else(|e| -1));
                    let event_line = event_line_watcher.update_infallible(event_line_pointer.deref(&process, &module, &image).unwrap_or_else(|e| -1));
                    let in_battle = in_battle_watcher.update_infallible(in_battle_pointer.deref(&process, &module, &image).unwrap_or_else(|e| false));
                    let battle_result = battle_result_watcher.update_infallible(battle_result_pointer.deref(&process, &module, &image).unwrap_or_else(|e| -1));
                    let adversaries_purified = adversaries_purified_watcher.update_infallible(adversaries_purified_pointer.deref(&process, &module, &image).unwrap_or_else(|e| false));
                    let mut zacharie_photo;
                    if let Ok(interpreter_name_wstr) = logic_interpreter_pointer.deref::<ArrayWString<32>>(&process, &module, &image) {
                        let interpreter_name = String::from_utf16_lossy(&interpreter_name_wstr);
                        zacharie_photo = zacharie_photo_watcher.update_infallible(interpreter_name == "photo de zacharie");
                        timer::set_variable("Interpreter Name",&interpreter_name);
                    } else {
                        zacharie_photo = zacharie_photo_watcher.update_infallible(false);
                        timer::set_variable("Interpreter Name","N/A");
                    };




                    timer::set_variable_int("Map ID",map_id.current);
                    timer::set_variable_int("Event ID",event_id.current);
                    timer::set_variable_int("Event Page",event_page.current);
                    timer::set_variable_int("Event Line",event_line.current);
                    timer::set_variable("In battle",&in_battle.current.to_string());
                    timer::set_variable("Zacharie Photo",&zacharie_photo.current.to_string());
                    timer::set_variable("Adversaries Purified",&adversaries_purified.current.to_string());
                    timer::set_variable_int("Battle Result",battle_result.current);

                    /*let scene = match scene_manager.get_current_scene_path::<128>(&process) {
                        Ok(path) => match String::from_utf8(Vec::from(get_name(&path))) {
                            Ok(name) => {scene_watcher.update_infallible(name)}
                            Err(_) => {scene_watcher.update_infallible("".to_string())}
                        },
                        Err(_) => {scene_watcher.update_infallible("".to_string())}
                    };
                    timer::set_variable("Scene",&scene.current);*/

                    if timer::state() == TimerState::Running || timer::state() == TimerState::Paused || timer::state() == TimerState::Ended {
                    }

                    if timer::state() == TimerState::NotRunning{
                        if !splits.is_empty() {
                            splits.clear();
                        }
                        if settings.autostart && map_id.current == 9 && event_id.current == 1 && event_page.current == 1 && event_line.old < 28 && event_line.current >= 28 {
                            timer::start();
                            asr::print_message("Timer has been started");
                        }
                    } else {
                        if settings.autoreset && map_id.current == 9 && event_id.current == 1 && event_page.current == 1 && event_line.old < 28 && event_line.current >= 28 {
                            timer::reset();
                            timer::start();
                            asr::print_message("Timer has been reset");
                            splits.clear();
                        }
                    }

                    if timer::state() == TimerState::Running {
                        if settings.zacharie_photo && zacharie_photo.current && !zacharie_photo.old && !splits.contains("zacharie_photo") {
                            split(&mut splits, "zacharie_photo");
                        }
                        //Bad Batter defeated
                        if settings.bad_batter && map_id.current == 347 && event_page.current == 5 && adversaries_purified.current && !adversaries_purified.old && !splits.contains("bad_batter") {
                            split(&mut splits, "bad_batter");
                        }

                        //most event-based splits
                        //tram-based splits are checked twice with different event IDs since each tram has two different doors
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.zone0, "zone0", 8, 1, 1, 12);
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.alpha_party_member, "alpha_joined", 17, 11, 1, 10);
                        check_event_exact_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                settings.enter_postal_service, "enter_postal", 34, 4, 1, 38);
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.alpha_party_member, "alpha_joined", 17, 11, 1, 10);
                        check_event_exact_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                settings.alma_second_half, "alma_second_half", 68, 3, 1, 4);
                        check_event_exact_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                settings.alma_second_half, "alma_second_half", 68, 4, 1, 4);
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.pure_zone1, "pure_zone1", 101, 1, 1, 10);
                        check_event_exact_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                settings.area1, "area1", 205, 5, 1, 3);
                        check_event_exact_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                settings.area1, "area1", 205, 6, 1, 3);
                        check_event_exact_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                settings.area2, "area2", 212, 5, 1, 13);
                        check_event_exact_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                settings.area2, "area2", 212, 6, 1, 13);
                        check_event_exact_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                settings.area3, "area3", 214, 3, 4, 4);
                        check_event_exact_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                settings.area3, "area3", 214, 5, 4, 4);
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.exit_the_room, "exit_the_room", 293, 1, 1, 10);
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.chapter2, "chapter2", 293, 6, 9, 6);
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.chapter1, "chapter1", 340, 1, 1, 0);
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.pure_zone2, "pure_zone2", 197, 1, 1, 10);
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.pure_zone3, "pure_zone3", 292, 1, 1, 10);
                        check_event_atleast_split(map_id.current, event_id.current, event_page.current, *event_line, &mut splits,
                                                  settings.ending_switch, "ending_switch", 347, 1, 1, 6);

                        //The Room early splits
                        check_event_new_page(map_id.current, event_id.current, *event_page, &mut splits,
                                             settings.chapter5, "chapter5", 293, 6, 3);
                        check_event_new_page(map_id.current, event_id.current, *event_page, &mut splits,
                                             settings.chapter4, "chapter4", 293, 6, 5);
                        check_event_new_page(map_id.current, event_id.current, *event_page, &mut splits,
                                             settings.chapter3, "chapter3", 293, 6, 7);

                        //alternative The Room splits - split on being sent back to the main room at the end of a chapter
                        /*check_map_split(*map_id, &mut splits,
                                        settings.chapter5, "chapter5", 297, 293);
                        check_map_split(*map_id, &mut splits,
                                        settings.chapter4, "chapter4", 310, 293);
                        check_map_split(*map_id, &mut splits,
                                        settings.chapter3, "chapter3", 331, 293);
                        check_map_split(*map_id, &mut splits,
                                        settings.chapter2, "chapter2", 334, 293);
                        check_map_split(*map_id, &mut splits,
                                        settings.chapter1, "chapter1", 339, 340);*/


                        //battle-based splits
                        win_battle_in_map_split(map_id.current, *battle_result, *in_battle, &mut splits,
                                                settings.uber1_source, "uber1_source", 356);
                        win_battle_in_map_split(map_id.current, *battle_result, *in_battle, &mut splits,
                                                settings.uber2_maldicion, "uber2_maldicion", 361);
                        win_battle_in_map_split(map_id.current, *battle_result, *in_battle, &mut splits,
                                                settings.uber3_psal_hero, "uber3_psal_hero", 357);
                        win_battle_in_map_split(map_id.current, *battle_result, *in_battle, &mut splits,
                                                settings.uber4_justus, "uber4_justus", 355);
                        win_battle_in_map_split(map_id.current, *battle_result, *in_battle, &mut splits,
                                                settings.uber5_carnival, "uber5_carnival", 359);
                        win_battle_in_map_split(map_id.current, *battle_result, *in_battle, &mut splits,
                                                settings.uber6_cob, "uber6_cob", 360);

                        //all map-change splits
                        check_map_split(*map_id, &mut splits,
                                        settings.enter_mines, "enter_mines", 19, 20);
                        check_map_split(*map_id, &mut splits,
                                        settings.mines, "mines", 23, 25);
                        check_map_split(*map_id, &mut splits,
                                        settings.barn, "barn", 28, 27);
                        check_map_split(*map_id, &mut splits,
                                        settings.postal_service, "postal_service", 47, 46);
                        check_map_split(*map_id, &mut splits,
                                        settings.alma_first_half, "alma_first_half", 57, 56);
                        check_map_split(*map_id, &mut splits,
                                        settings.zone1, "zone1", 69, 70);
                        check_map_split(*map_id, &mut splits,
                                        settings.card_puzzle, "card_puzzle", 114, 112);
                        if battle_result.current == 5 { //post-battle split
                            check_map_split(*map_id, &mut splits,
                                            settings.japhet1, "japhet1", 117, 116);
                            check_map_split(*map_id, &mut splits,
                                            settings.sugar, "japhet1", 152, 151);
                        }
                        check_map_split(*map_id, &mut splits,
                                        settings.park, "park", 136, 134);
                        check_map_split(*map_id, &mut splits,
                                        settings.residential, "residential", 145, 115);
                        check_map_split(*map_id, &mut splits,
                                        settings.zone2, "zone2", 162, 70);
                        check_map_split(*map_id, &mut splits,
                                        settings.critic_burnt, "critic_burnt", 234, 213);
                        check_map_split(*map_id, &mut splits,
                                        settings.area4, "area4", 235, 213);
                        check_map_split(*map_id, &mut splits,
                                        settings.enoch, "enoch", 213, 2);
                    }
                    timer::set_variable_int("Completed Splits",splits.len());

                    next_tick().await;
                };
            })
            .await;




    }
}
