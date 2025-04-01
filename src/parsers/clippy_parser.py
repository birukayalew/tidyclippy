import re
from collections import defaultdict
from categories.clippy_categories import CLIPPY_CATEGORY_MAP

# List to store messages that couldn't be categorized.
uncategorized_log = []

def categorize_clippy(message):
    message_lower = message.lower()
    # Use predefined keywords.
    for category, keywords in CLIPPY_CATEGORY_MAP.items():
        for keyword in keywords:
            if keyword in message_lower:
                return category
    uncategorized_log.append(message)
    return "Uncategorized"

# Parses clippy output and organizes issues by file and category.
def parse_clippy_output(lines):
    issues_by_file = defaultdict(lambda: defaultdict(list)) # Dictionary to store issues organized by file and category.
    current_file = None
    current_line = None

    # 1. Matches lint messages (warning/error with optional code).
    lint_pattern = re.compile(r'^(warning|error)(\[\w+\])?: (.+)')
    # 2. Matches file locations (--> path:line:column).
    location_pattern = re.compile(r'^\s*-->\s*(.+?):(\d+):\d+')

    current_issue = None

    for line in lines:
        line = line.strip()
        
        # Check for file location lines.
        location_match = location_pattern.match(line)
        if location_match:
            current_file, line_no = location_match.groups()
            current_line = int(line_no)
            continue

        lint_match = lint_pattern.match(line)
        if lint_match and current_file:
            level, message = lint_match.group(1), lint_match.group(3)
            # Skipping summary lines.
            if re.match(r"^`(.+)` \(bin \"(.+)\"\) generated (\d+) (warning|warnings)$"
, message):
                continue
            if re.match(r'^could not compile', message):
                continue
            category = categorize_clippy(message)

            current_issue = {
                'line': current_line,
                'level': level,
                'message': message
            }
            issues_by_file[current_file][category].append(current_issue)

    return issues_by_file, uncategorized_log