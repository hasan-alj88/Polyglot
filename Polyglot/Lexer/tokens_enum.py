from enum import auto, Enum, IntEnum
from pydantic import BaseModel
from typing import List, Optional
from pregex.core.classes import AnyDigit, AnyLetter
from pregex.core.quantifiers import Optional, OneOrMore,

class TokenType(IntEnum):
    SQUARE_ELEMENT = auto()
    KEYWORD = auto()
    OPERATOR = auto()
    IDENTIFIER = auto()
    Literals = auto()
    NUMBER = auto()


class TokenInfo(BaseModel):
    string_list: Optional[str | List[str]]
    first_char_regex: str | List[str]
    last_char_regex: str | List[str]
    regex: str
    type: TokenType
    description: str = ""


class Tokens(Enum):
    INTEGER = TokenInfo(
        string_list=None,
        first_char_regex=AnyDigit().get_pattern(),
        last_char_regex=AnyDigit().get_pattern(),
        regex=OneOrMore(AnyDigit()).get_pattern(),
        type=TokenType.NUMBER,
        description="Integer number"
    )
    FLOAT = TokenInfo(
        string_list=None,
        first_char_regex="[0-9\.]",
        last_char_regex="[0-9\.]",
        type=TokenType.NUMBER,
        regex="[0-9]+\.[0-9]+",
    )
    STRING_LITERAL = TokenInfo(
        string_list=None,
        first_char_regex="\"",
        last_char_regex="\"",
        regex="\"[^\"]*\"",
        type=TokenType.Literals,
        description="String literal"
    )
    DATATYPE = TokenInfo(
        string_list=None,
        first_char_regex="[a-zA-Z]",
        last_char_regex="[\x20-\x7E]",
        regex="^[a-zA-Z]+[\/][\x20-\x7E]+",
        type=TokenType.KEYWORD,
        description="Data type"
    )