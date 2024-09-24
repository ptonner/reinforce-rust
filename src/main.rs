use std::collections::HashSet;

trait IEnvironment {
    type Action;
    type State;

    /// The full dynamics of the environment: the probability of
    /// transitioning from state `from` to state `to`, taking action
    /// `take` and receiving rewards `with`.
    fn prob(from: &Self::State, take: &Self::Action, to: &Self::State, with: &f32) -> f32;

    // Expectations

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

    /// Calculate the expected reward taking action `take` from state
    /// `from`, marginalizing over all possible resulting states.
    fn expected_reward(from: &Self::State, take: &Self::Action) -> Option<f32> {
        let rewards = Self::rewards();
        if rewards.len() > 0 {
            let to = Self::states_from(from, take);
            return Some(
                rewards
                    .iter()
                    .flat_map(|r| to.iter().map(move |t| (r, t)))
                    .map(|(r, t)| Self::prob(from, take, t, r) * r)
                    .sum(),
            );
        } else {
            None
        }
    }

    /// Calculate the expected reward taking action `take` from state
    /// `from`, arriving at state `to`.
    fn expected_reward_at(
        from: &Self::State,
        take: &Self::Action,
        to: &Self::State,
    ) -> Option<f32> {
        let rewards = Self::rewards();
        if rewards.len() > 0 {
            Some(
                rewards
                    .iter()
                    .map(|r| Self::prob(from, take, to, r) * r)
                    .sum(),
            )
        } else {
            None
        }
    }

    // Space enumeration functions:

    fn actions_from(from: &Self::State) -> HashSet<Self::Action>;
    fn states_from(from: &Self::State, take: &Self::Action) -> HashSet<Self::State>;

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
#[derive(Debug, Hash, Eq, PartialEq)]
enum Always {
    Same,
}

#[derive(Debug)]
struct Dull;
impl IEnvironment for Dull {
    type State = Always;
    type Action = DoNothing;

    fn actions_from(from: &Self::State) -> HashSet<Self::Action> {
        HashSet::from_iter(vec![DoNothing::Nothing])
    }
    fn states_from(from: &Self::State, take: &Self::Action) -> HashSet<Self::State> {
        HashSet::from_iter(vec![Always::Same])
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
    println!("Available actions: {:?}", Dull::actions_from(&init));
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
    );
    println!(
        "Expected reward: {:?}",
        Dull::expected_reward(&init, &DoNothing::Nothing)
    );
    println!(
        "Expected reward at: {:?}",
        Dull::expected_reward_at(&init, &DoNothing::Nothing, &init)
    );
}
