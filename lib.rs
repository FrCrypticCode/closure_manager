#![no_std]
#![allow(warnings)]
extern crate alloc;
use core::result::Result;
use alloc::{boxed::Box,collections::VecDeque, vec::Vec};
use core::ops::{Deref, DerefMut};

/// Gestionnaire de closure<br>
/// Un ensemble de méthodes est inclus pour gérer le VecDeque de closures.
/// Il est préférable de recourir à celles-ci pour limiter les extensions de capacité
/// et éviter tout risque d'accès concurentiel à une même closure.
pub struct Manager{
    list:VecDeque<Slot>
}
/// Accès immédiat à l'unique propriété
impl Deref for Manager{
    type Target = VecDeque<Slot>;
    fn deref(&self) -> &Self::Target{
        &self.list
    }
}
impl DerefMut for Manager{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list
    }
}
impl Manager{
    /// Le Manager requiert une taille lors de sa construction pour définir une capacité initiale.
    pub fn new(cap:usize)->Manager{
        return Manager { list: VecDeque::with_capacity(cap) }
    }
    /// Ajout d'un Slot<br>
    /// La méthode retourne un numéro d'index.
    pub fn add(&mut self,func:Slot)->usize{
        let mut n = -1;
        for (id,f) in self.list.iter_mut().enumerate(){
            if f.used == Status::Used || f.used == Status::Empty{
                n = id as i32;
                break;
            }
        }
        if n != -1{
            self.list[n as usize] = func;
            return n as usize
        }
        else{
            self.list.push_back(func);
            return (self.list.len()-1) as usize
        }
    }
    /// Libération d'un emplacement, ce dernier passe en Empty et pourra être utilisé pour insérer
    /// une autre closure.
    pub fn rem(&mut self,id:usize){
        self.list[id].closure = Box::new(||{});
        self.list[id].used =Status::Empty;
    }

    /// Passage d'un status en Pending (Prêt à l'emploi avec exec).
    pub fn reload(&mut self,id:usize){
        self.list[id].used = Status::Pending;
    }
    pub fn reload_all(&mut self){
        for f in self.list.iter_mut(){
            f.used = Status::Pending;
        }
    }

    /// Appel à exécution d'une closure<br>
    /// Une closure est utilisable une seule fois mais peut être remise en utilisation via la méthode reload.
    /// Un booléen est renvoyé pour préciser si l'exécution a été initié, un false = Pas de closure ou une closure en cours d'usage.
    /// L'implémentation d'exec a pour but de s'assurer qu'aucun accès concurrentiel ne soit effectif sur du travail Async/Multi-thread.
    pub fn exec(&mut self,id:usize)->bool{
        if self.list[id].used != Status::Busy && self.list[id].used != Status::Empty{
            self.list[id].used = Status::Busy;
            (self.list[id])();
            self.list[id].used = Status::Used;
            return true
        } 
        return false
    }
    /// Renvoi un vecteur d'index si la méthode rencontre des problèmes.
    pub fn exec_all(&mut self)->Result<(),Vec<usize>>{
        let mut errs:Vec<usize> = Vec::new();
        for (id,f) in self.list.iter_mut().enumerate(){
            if f.used == Status::Busy || f.used == Status::Empty{
                errs.push(id);
                continue;
            }
            f.used = Status::Busy;
            (f)();
            f.used = Status::Used;
        }
        if errs.len() != 0{
            return Err(errs)
        }
        return Ok(())
    }
}

/// Ensemble des status attribuables sur un Slot<br>
/// Un Slot arrive en Pending, passe en Busy durant tout usage et termine en Used.
/// La variante Empty est utilisé pour libérer sans qu'aucun appel n'ait été fait.
#[derive(PartialEq,Debug)]
pub enum Status{
    Pending,
    Busy,
    Used,
    Empty
}

/// Le Slot correspond à une closure en attente d'exécution<br>
/// Chaque Slot dispose d'un Status via l'index used et d'un trait Deref pour un accès immédiat.
/// Un Slot peut être exécuté manuellement selon la méthode suivante => (Slot)()
pub struct Slot{
    closure : Box<dyn Fn()>,
    pub used : Status
}
impl Deref for Slot{
    type Target = Box<dyn Fn()>;
    fn deref(&self) -> &Self::Target {
        &self.closure
    }
}
impl Slot{
    pub fn new(func:Box<dyn Fn()>)->Slot{
        return Slot { closure: func, used: Status::Pending }
    }
}

