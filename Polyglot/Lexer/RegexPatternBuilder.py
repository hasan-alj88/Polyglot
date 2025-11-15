import re

class RegexPatternBuilder:
    def __init__(self):
        self.patterns = []

    def add_literal(self, text):
        self.patterns.append(re.escape(text))
        return self

    def add_options(self, options):
        escaped = [re.escape(opt) for opt in options]
        self.patterns.append(f"(?:{'|'.join(sorted(escaped, key=len, reverse=True))})")
        return self

    def zero_or_more(self, pattern):
        self.patterns.append(f"({re.escape(pattern)})*")
        return self

    def one_or_more(self, pattern):
        self.patterns.append(f"({re.escape(pattern)})+")
        return self

    def optional_literal(self, text):
        self.patterns.append(f"(?:{re.escape(text)})?")
        return self

    def optional_options(self, options):
        escaped = [re.escape(opt) for opt in options]
        self.patterns.append(f"(?:(?:{'|'.join(sorted(escaped, key=len, reverse=True))}))?")
        return self

    def bracketed_options(self, options):
        """Specifically for [option] patterns"""
        escaped = [re.escape(opt) for opt in options]
        pattern = f"\\[(?:{'|'.join(sorted(escaped, key=len, reverse=True))})\\]"
        self.patterns.append(f"(?:{pattern})?")
        return self

    def raw(self, pattern):
        """Add raw regex without escaping"""
        self.patterns.append(pattern)
        return self

    def build(self) -> str:
        return re.compile("^" + "".join(self.patterns)).pattern

    def build_pattern_string(self):
        """Return just the pattern string without compiling"""
        return "^" + "".join(self.patterns)