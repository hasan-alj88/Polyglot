from dataclasses import dataclass

from Polyglot.Lexer.tokens_enum import Tokens


@dataclass
class Token:
    token: Tokens
    value: str
    line: int
    column: int

    def __len__(self):
        return len(self.value)

    def __str__(self):
        return f"{self.token.name}[{self.value}]"
    def __repr__(self):
        return f"{self.token.name}[{self.value}]"