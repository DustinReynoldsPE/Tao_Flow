//! The vessel is not the water. It is the boat you ride,
//! the window through which you see the mountain.
//!
//! tmux provides the walls of the space each agent occupies.
//! Each window allows the user to perceive the agent --
//! much like you can see the mountain outside of a window,
//! but the window is not the mountain.
//!
//! "Thirty spokes share the wheel's hub;
//!  It is the center hole that makes it useful.
//!  Shape clay into a vessel;
//!  It is the space within that makes it useful."
//!  -- Tao Te Ching, Chapter 11
//!
//! The vessel manages persistent tmux sessions where Claude
//! processes live. Springs ride in vessels. The vessel carries
//! the conversation naturally -- the spring does not need to
//! carry its own memory.

pub mod tmux;

pub use tmux::TmuxVessel;
