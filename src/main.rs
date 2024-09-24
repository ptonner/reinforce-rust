use std::collections::HashSet;

trait IEnvironment {
    type Action;
    type State;

    fn available_actions(from: &Self::State) -> HashSet<Self::Action>;

    /// The full dynamics of the environment: the probability of
    /// transitioning from state `from` to state `to`, taking action
    /// `take` and receiving rewards `with`.
    fn prob(from: &Self::State, take: &Self::Action, to: &Self::State, with: &f32) -> f32;

    /// Calculates the probability of transitioning from state `from`
    /// to state `to`, taking action `take`.
    ///
    /// This is formally calcuated as the marginal probability over
    /// different rewards values. If `rewards` is implemented, then
    /// this probability can be calculated exactly, and is provided
    /// through the trait's default implementation.
    ///
    /// If `rewards` is not or cannot be implemented (e.g. for an
    /// infinite range), then this function must be defined explitly
    fn prob_transition(from: &Self::State, take: &Self::Action, to: &Self::State) -> Option<f32> {
        let rewards = Self::rewards();
        if rewards.len() > 0 {
            return Some(rewards.iter().map(|s| Self::prob(from, take, to, s)).sum());
        } else {
            None
        }
    }

    // fn expected_reward(from: &Self::State, )

    /// Enumerates the possible reward values in the environment
    /// (optional)
    fn rewards() -> Vec<f32> {
        vec![]
    }
}

// Example interface implementations
#[derive(Hash, Eq, PartialEq, Debug)]
enum DoNothing {
    Nothing,
}
#[derive(Debug)]
enum Always {
    Same,
}

#[derive(Debug)]
struct Dull;
impl IEnvironment for Dull {
    type State = Always;
    type Action = DoNothing;

    fn available_actions(from: &Self::State) -> HashSet<Self::Action> {
        HashSet::from_iter(vec![DoNothing::Nothing])
    }
    fn rewards() -> Vec<f32> {
        vec![0.0]
    }
    fn prob(_: &Self::State, _: &Self::Action, _: &Self::State, with: &f32) -> f32 {
        if *with == 0.0 {
            1.0
        } else {
            0.0
        }
    }
}

fn main() {
    let init = Always::Same;
    println!("Available actions: {:?}", Dull::available_actions(&init));
    println!(
        "Prob: {:?}",
        Dull::prob(&init, &DoNothing::Nothing, &init, &0.0)
    );
    println!(
        "Prob: {:?}",
        Dull::prob(&init, &DoNothing::Nothing, &init, &1.0)
    );
    println!(
        "Transition prob: {:?}",
        Dull::prob_transition(&init, &DoNothing::Nothing, &init)
    )
}
