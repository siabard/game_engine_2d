//! # Build 2D game engine using SDL2
//! Inspired by [Making A Game #1: Making The Game Loop : C++ And SDL2 Tutorial](https://www.youtube.com/watch?v=44tO977slsU)

#![deny(missing_docs)]

extern crate sdl2;
extern crate uuid;

pub mod app;
pub mod sdl_engine;
pub mod map;
pub mod ecs;
pub mod components;
