from enum import StrEnum
from typing import Any

VICTORY_POINTS = 6
DRAW_POINTS = 3
DEFEAT_POINTS = 0


class Plays(StrEnum):
    ROCK = "rock"
    PAPER = "paper"
    SCISSOR = "scissor"


class Result(StrEnum):
    WIN = "win"
    DRAW = "draw"
    LOSS = "loss"


class StrategyDecoder:
    def __init__(
        self,
        play_map: dict[str, Plays],
        rules: dict[Plays, dict[str, str]],
        outcome_map: dict[str, Result],
    ) -> None:
        self.play_map = play_map
        self.rules = rules
        self.outcome_map = outcome_map

    def decode_play(self, play: str) -> Plays:
        return self.play_map[play]

    def decode_result(self, result: str) -> Plays:
        return self.outcome_map[result]

    def choose_play(self, opponent_play: Plays, outcome: Result) -> Plays:
        if outcome == Result.WIN:
            return self.rules[opponent_play][Result.LOSS]
        elif outcome == Result.DRAW:
            return self.rules[opponent_play][Result.DRAW]
        else:
            return self.rules[opponent_play][Result.WIN]


class Game:
    def __init__(
        self,
        rules: dict[Plays, dict[str, str]],
        rounds: list[str, str],
        decoder: StrategyDecoder,
    ):
        self.rules = rules
        self.rounds = rounds
        self.strategy_decoder = decoder
        self.game_score = [0, 0]  # opponent, me

    def play(self) -> None:
        for round in self.rounds:
            opponent_play = self.strategy_decoder.decode_play(round[0])
            outcome = self.strategy_decoder.decode_result(round[1])
            my_play = self.strategy_decoder.choose_play(opponent_play, outcome)

            my_play_rules = self.rules[my_play]
            opponent_rules = self.rules[opponent_play]

            if self.rules[opponent_play][Result.LOSS] == my_play:  # I Won
                self.game_score[0] += my_play_rules["score"] + VICTORY_POINTS
                self.game_score[1] += opponent_rules["score"] + DEFEAT_POINTS
            elif self.rules[opponent_play][Result.DRAW] == my_play:  # Draw
                self.game_score[0] += my_play_rules["score"] + DRAW_POINTS
                self.game_score[1] += opponent_rules["score"] + DRAW_POINTS
            elif self.rules[opponent_play][Result.WIN] == my_play:  # I lost
                self.game_score[0] += my_play_rules["score"] + DEFEAT_POINTS
                self.game_score[1] += opponent_rules["score"] + VICTORY_POINTS

    def get_game_score(self) -> str:
        return (
            f" My Score: {self.game_score[0]} \n Opponent Score: {self.game_score[1]}"
        )


if __name__ == "__main__":
    with open("./example_input", "rb") as f:
        play_list = []
        for line in f:
            line_str = line.strip().decode("utf-8")
            play_array = line_str.split(" ")
            play_list.append(play_array)

    strategy_map = {
        "A": Plays.ROCK,
        "B": Plays.PAPER,
        "C": Plays.SCISSOR,
    }

    result_map = {"X": Result.LOSS, "Y": Result.DRAW, "Z": Result.WIN}

    game_rules = {
        Plays.ROCK: {
            "score": 1,
            Result.WIN: Plays.SCISSOR,
            Result.LOSS: Plays.PAPER,
            Result.DRAW: Plays.ROCK,
        },
        Plays.PAPER: {
            "score": 2,
            Result.WIN: Plays.ROCK,
            Result.LOSS: Plays.SCISSOR,
            Result.DRAW: Plays.PAPER,
        },
        Plays.SCISSOR: {
            "score": 3,
            Result.WIN: Plays.PAPER,
            Result.LOSS: Plays.ROCK,
            Result.DRAW: Plays.SCISSOR,
        },
    }

    strategy_decoder = StrategyDecoder(strategy_map, game_rules, result_map)

    game = Game(game_rules, play_list, strategy_decoder)
    game.play()

    print(game.get_game_score())
