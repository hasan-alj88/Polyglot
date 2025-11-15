import unittest


def convert_to_full_format(shorthand):
    """
    Convert a shorthand timestamp format to full format T"YYYY-MM-DDThh:mm:ss.000000"
    Expected order: year-month-dayThour:minute:second.nanoseconds
    Components are identified by their position relative to separators
    """
    # First check if the string of form T"*" where * is some string, then get that string
    if shorthand.startswith('T"') and shorthand.endswith('"'):
        shorthand = shorthand[2:-1]
    else:
        raise ValueError("Invalid timestamp format, it must be of the form T\"YYYY-MM-DDThh:mm:ss.000000\"")

    import re

    # Initialize result components with defaults
    components = ["0000", "00", "00", "00", "00", "00", "000000"]  # Y, M, D, h, m, s, ns

    # Parse based on separators and positions
    remaining = shorthand
    pos = 0  # Which component position we're at

    # Find separators and split accordingly
    separators = ['-', '-', 'T', ':', ':', '.']

    i = 0
    while i < len(remaining) and pos < 7:
        if pos < 6:  # Not the last component
            # Look for the expected separator
            expected_sep = separators[pos]
            sep_idx = remaining.find(expected_sep, i)

            if sep_idx != -1:
                # Found separator, extract component before it
                component_str = remaining[i:sep_idx]
                if component_str.isdigit():
                    if pos == 0:  # Year
                        components[pos] = component_str.zfill(4)
                    elif pos == 6:  # Nanoseconds
                        components[pos] = component_str.ljust(6, '0')[:6]
                    else:
                        components[pos] = component_str.zfill(2)

                i = sep_idx + 1  # Move past separator
                pos += 1
            else:
                # No separator found, check if we have digits at current position
                component_str = ""
                j = i
                while j < len(remaining) and remaining[j].isdigit():
                    component_str += remaining[j]
                    j += 1

                if component_str:
                    if pos == 0:  # Year
                        components[pos] = component_str.zfill(4)
                    elif pos == 6:  # Nanoseconds
                        components[pos] = component_str.ljust(6, '0')[:6]
                    else:
                        components[pos] = component_str.zfill(2)
                break
        else:  # Last component (nanoseconds)
            component_str = remaining[i:]
            if component_str.isdigit():
                components[pos] = component_str.ljust(6, '0')[:6]
            break

    # Handle special cases based on separator patterns
    if shorthand.startswith('-'):
        # Starts with -, so first number is month
        numbers = re.findall(r'\d+', shorthand)
        components = ["0000", "00", "00", "00", "00", "00", "000000"]
        if len(numbers) > 0:
            components[1] = numbers[0].zfill(2)  # Month
    elif 'T' in shorthand and not any(c in shorthand for c in ['-']):
        # Contains T but no dashes, so it's day
        numbers = re.findall(r'\d+', shorthand)
        components = ["0000", "00", "00", "00", "00", "00", "000000"]
        if len(numbers) > 0:
            components[2] = numbers[0].zfill(2)  # Day
        if len(numbers) > 1:
            components[3] = numbers[1].zfill(2)  # Hour
    elif shorthand.count(':') == 2 and '.' in shorthand:
        # Pattern like "1::." means hour::second
        numbers = re.findall(r'\d+', shorthand)
        components = ["0000", "00", "00", "00", "00", "00", "000000"]
        if len(numbers) > 0:
            components[3] = numbers[0].zfill(2)  # Hour
        if len(numbers) > 1:
            components[5] = numbers[1].zfill(2)  # Second
    elif shorthand.count(':') == 1 and '.' in shorthand:
        # Pattern like "1:." means minute:second
        numbers = re.findall(r'\d+', shorthand)
        components = ["0000", "00", "00", "00", "00", "00", "000000"]
        if len(numbers) > 0:
            components[4] = numbers[0].zfill(2)  # Minute
    elif '.' in shorthand and ':' not in shorthand:
        # Pattern like "1." means second
        numbers = re.findall(r'\d+', shorthand)
        components = ["0000", "00", "00", "00", "00", "00", "000000"]
        if len(numbers) > 0:
            components[5] = numbers[0].zfill(2)  # Second
        if len(numbers) > 1:
            components[6] = numbers[1].ljust(6, '0')[:6]  # Nanoseconds
    elif '--' in shorthand:
        # Pattern like "1--" means year with empty month/day
        numbers = re.findall(r'\d+', shorthand)
        if len(numbers) > 0:
            components[0] = numbers[0].zfill(4)  # Year
    else:
        # Standard parsing
        pass

    # Build the full format string
    full_format = f"{components[0]}-{components[1]}-{components[2]}T{components[3]}:{components[4]}:{components[5]}.{components[6]}"

    return f'T"{full_format}"'


class TestConvertToFullFormat(unittest.TestCase):

    def test_nanosecond(self):
        """1 ns = T"1" (just number, goes to year by default)"""
        result = convert_to_full_format('T"1"')
        expected = 'T"0001-00-00T00:00:00.000000"'
        self.assertEqual(result, expected)

    def test_millisecond(self):
        """1000 ns = T"1000" (just number, goes to year by default)"""
        result = convert_to_full_format('T"1000"')
        expected = 'T"1000-00-00T00:00:00.000000"'
        self.assertEqual(result, expected)

    def test_second(self):
        """1 s = T"1." (number with dot = seconds)"""
        result = convert_to_full_format('T"1."')
        expected = 'T"0000-00-00T00:00:01.000000"'
        self.assertEqual(result, expected)

    def test_minute(self):
        """1 m = T"1:." (minute:second pattern)"""
        result = convert_to_full_format('T"1:."')
        expected = 'T"0000-00-00T00:01:00.000000"'
        self.assertEqual(result, expected)

    def test_hour(self):
        """1 h = T"1::." (hour::second pattern)"""
        result = convert_to_full_format('T"1::."')
        expected = 'T"0000-00-00T01:00:00.000000"'
        self.assertEqual(result, expected)

    def test_day(self):
        """1 day = T"1T" (day with T separator)"""
        result = convert_to_full_format('T"1T"')
        expected = 'T"0000-00-01T00:00:00.000000"'
        self.assertEqual(result, expected)

    def test_month(self):
        """1 month = T"-1-" (starts with -, so month)"""
        result = convert_to_full_format('T"-1-"')
        expected = 'T"0000-01-00T00:00:00.000000"'
        self.assertEqual(result, expected)

    def test_year(self):
        """1 year = T"1--" (year with -- pattern)"""
        result = convert_to_full_format('T"1--"')
        expected = 'T"0001-00-00T00:00:00.000000"'
        self.assertEqual(result, expected)

    def test_year_month(self):
        """1 year 1 month = T"1-1" """
        result = convert_to_full_format('T"1-1"')
        expected = 'T"0001-01-00T00:00:00.000000"'
        self.assertEqual(result, expected)

    def test_day_second(self):
        """1 day 4 s = T"1T1." (day T hour, but second component)"""
        result = convert_to_full_format('T"1T1."')
        expected = 'T"0000-00-01T01:00:00.000000"'
        self.assertEqual(result, expected)

    def test_hours_seconds(self):
        """11 hours 2s = T"11::2" (hour::second)"""
        result = convert_to_full_format('T"11::2"')
        expected = 'T"0000-00-00T11:00:02.000000"'
        self.assertEqual(result, expected)

    def test_full_datetime(self):
        """Full datetime"""
        result = convert_to_full_format('T"2024-12-25T14:30:45.123456"')
        expected = 'T"2024-12-25T14:30:45.123456"'
        self.assertEqual(result, expected)

    def test_partial_datetime(self):
        """Partial datetime - year month day"""
        result = convert_to_full_format('T"2024-12-25"')
        expected = 'T"2024-12-25T00:00:00.000000"'
        self.assertEqual(result, expected)


if __name__ == '__main__':
    unittest.main(verbosity=2)