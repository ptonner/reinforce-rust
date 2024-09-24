use std::collections::HashSet;

trait IState {
    /// Type enumerating possible actions possible from all states
    type Action;

    fn available_actions(&self) -> HashSet<Self::Action>;
}

trait IEnvironment {
    type Action;
    type State: IState;

    /// The full dynamics of the environment: the probability of transitioning from state `from` to state `to`, taking action `take` and receiving rewards `with`.
    fn prob(from: &Self::State, take: &Self::Action, to: &Self::State, with: &f32) -> f32;

    /// Calculates the probability of transitioning from state `from` to state `to`, taking action `take`.
    ///
    /// This is formally calcuated as the marginal probability over different rewards values. If `rewards` is implemented, then this probability can be calculated exactly, and is provided through the trait's default implementation.
    ///
    /// If `rewards` is not or cannot be implemented (e.g. for an infinite range), then this function must be defined explitly
    fn prob_transition(from: &Self::State, take: &Self::Action, to: &Self::State) -> Option<f32> {
        let rewards = Self::rewards();
        if rewards.len() > 0 {
            return Some(rewards.iter().map(|s| Self::prob(from, take, to, s)).sum());
        } else {
            None
        }
    }

    /// Enumerates the possible reward values in the environment (optional)
    fn rewards() -> Vec<f32> {
        vec![]
    }
}

// Example interface implementations
#[derive(Hash, Eq, PartialEq, Debug)]
enum DoNothing {
    nothing,
}
#[derive(Debug)]
struct State;
impl IState for State {
    type Action = DoNothing;
    fn available_actions(&self) -> HashSet<Self::Action> {
        HashSet::from_iter(vec![DoNothing::nothing])
    }
}
#[derive(Debug)]
struct Dull;
impl IEnvironment for Dull {
    type State = State;
    type Action = DoNothing;

    fn rewards() -> Vec<f32> {
        vec![0.0]
    }
    fn prob(from: &Self::State, take: &Self::Action, to: &Self::State, with: &f32) -> f32 {
        if *with == 0.0 {
            1.0
        } else {
            0.0
        }
    }
}

fn main() {
    let init = State {};
    println!("Available actions: {:?}", init.available_actions());
    println!(
        "Prob: {:?}",
        Dull::prob(&init, &DoNothing::nothing, &init, &0.0)
    );
    println!(
        "Prob: {:?}",
        Dull::prob(&init, &DoNothing::nothing, &init, &1.0)
    );
    println!(
        "Transition prob: {:?}",
        Dull::prob_transition(&init, &DoNothing::nothing, &init)
    )
}
