import itertools
import json
import re
from pathlib import Path

from Polyglot.Lexer.RegexPatternBuilder import RegexPatternBuilder


def load_polyglot_element_data() -> dict:
    file_path = Path(__file__).parent / "polyglot_element_data.json"
    with open(file_path, 'r') as f:
        return json.load(f)


def build_square_elements_pattern() -> re.Pattern:
    raw_elements: dict = load_polyglot_element_data()["square_elements"]
    raw_elements: list = [elements for _,elements in raw_elements.items()]
    raw_elements: list = itertools.chain.from_iterable(raw_elements)
    tokens = [e for e in raw_elements if e and isinstance(e, str)]

    return (RegexPatternBuilder()
            .zero_or_more("[~]")
            .bracketed_options(tokens)
            .build())

def main():
    print(build_square_elements_pattern())
    # input_str = "[~][~][~][r] Something [Else]"
    # pattern = r"^(\[[^\]]+\])"
    # sqr_elements = re.findall(pattern, input_str)
    # print(sqr_elements)
    # for element in sqr_elements:
    #     print(element)

if __name__ == "__main__":
    main()