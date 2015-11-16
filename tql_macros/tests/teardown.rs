/*
 * Copyright (C) 2015  Boucher, Antoni <bouanto@zoho.com>
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Tear-down object to execute a closure at the end of the test.

pub struct TearDown<F: Fn()> {
    closure: F,
}

impl<F: Fn()> TearDown<F> {
    pub fn new(closure: F) -> TearDown<F> {
        TearDown {
            closure: closure,
        }
    }
}

impl<F: Fn()> Drop for TearDown<F> {
    fn drop(&mut self) {
        (self.closure)();
    }
}