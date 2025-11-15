from pydantic import BaseModel
from typing import List, Self
from Polyglot.Lexer.tokens import Token


class Tree(BaseModel):
    node: Token
    children: List[Self]