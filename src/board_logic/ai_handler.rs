//! This module is the main entrance point to the asynchronous ai. It spawns the worker thread and takes care
//! of the communication.

use crate::board_logic::alpha_beta::AlphaBeta;
use crate::board_logic::bit_board::{BitBoard};
use std::sync::mpsc;
use std::thread;

/// The handle struct is the entry point to the ai, where one can request
/// things and can obtain the result.
pub struct AiHandler {
    receiver: mpsc::Receiver<u32>,
    sender: mpsc::Sender<BitBoard>,
}

impl AiHandler {
    /// The constructor spawns a new thread for the ai calculation and keeps a channel pair.
    pub fn new() -> AiHandler {
        let (result_sender, result_receiver) = mpsc::channel::<u32>();
        let (request_sender, request_receiver) = mpsc::channel::<BitBoard>();

        // Kick off worker thread.
        // Kick of a worker thread, that runs in the background.
        thread::spawn(move || {
            let mut ai = AlphaBeta::new();
            loop {
                let local_board = request_receiver.recv().unwrap();
                let result = ai.get_best_move(local_board);
                let content = result_sender.send(result);
                content.unwrap();
            }
        });

        AiHandler {
            receiver: result_receiver,
            sender: request_sender,
        }
    }

    /// Send a request over to the thread, as the board will be consumed by the
    /// channel, you will have to clone it upfront, if you want to keep it.
    pub fn send_analysis_request(&self, board: BitBoard) {
        self.sender
            .send(board)
            .expect("AiHandler failed to send analysis request");
    }

    /// Tries to get an answer from the thread, if there is still no available None
    /// is returned.
    pub fn try_get_computation_result(&self) -> Option<u32> {
        self.receiver.try_recv().ok()
    }
}
