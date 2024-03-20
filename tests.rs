mod lib;
use crate::lib::{Manager,Slot,Status};

#[test]
fn build(){
    let mut list = Manager::new(5);
    let slot_01 = Slot::new(Box::new(||{let mut x = 0;x+= 2;}));
    let id1 = list.add(slot_01);
    assert_eq!(id1,0);
}

#[test]
fn test_inserts(){
    let mut list = Manager::new(5);
    let slot_01 = Slot::new(Box::new(||{let mut x = 0;x+= 2;}));
    let mut slot_02 = Slot::new(Box::new(||{let y = 5; let i = 0;}));
    slot_02.used = Status::Busy;
    let id1 = list.add(slot_01);
    list.push_back(slot_02);
    assert_eq!(list[1].used,Status::Busy);
}

#[test]
fn exec_ok(){
    let mut list = Manager::new(5);
    let slot_01 = Slot::new(Box::new(||{let mut x = 0;x+= 2;}));
    let slot_02 = Slot::new(Box::new(||{let y = 5; let i = 0;}));
    list.add(slot_01);
    list.push_back(slot_02);
    assert_eq!(list.exec(1),true);
}

#[test]
fn exec_err(){
    let mut list = Manager::new(5);
    let slot_01 = Slot::new(Box::new(||{let mut x = 0;x+= 2;}));
    let mut slot_02 = Slot::new(Box::new(||{let y = 5; let i = 0;}));
    slot_02.used = Status::Busy;
    let id1 = list.add(slot_01);
    list.push_back(slot_02);
    assert_eq!(list.exec(1),false);
}

#[test]
fn exec_all(){
    let mut list = Manager::new(5);
    let slot_01 = Slot::new(Box::new(||{let mut x = 0;x+= 2;}));
    let mut slot_02 = Slot::new(Box::new(||{let y = 5; let i = 0;}));
    slot_02.used = Status::Busy;
    let id1 = list.add(slot_01);
    list.push_back(slot_02);
    let l = list.exec_all();
    assert_eq!(l.is_ok(),false);
    assert_eq!(l.unwrap_err().len(),1);
}

#[test]
fn remove(){
    let mut list = Manager::new(5);
    let slot_01 = Slot::new(Box::new(||{let mut x = 0;x+= 2;}));
    let mut slot_02 = Slot::new(Box::new(||{let y = 5; let i = 0;}));
    slot_02.used = Status::Busy;
    let id1 = list.add(slot_01);
    list.push_back(slot_02);
    list.rem(0);
    assert_eq!(list[0].used,Status::Empty);
}