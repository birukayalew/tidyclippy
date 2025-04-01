import re
from categories.clang_categories import CLANG_CATEGORY_MAP
from collections import defaultdict

# List to store messages that couldn't be categorized.
uncategorized_log = []

def categorize(message):
    message_lower = message.lower()
    # Use predefined keywords.
    for category, keywords in CLANG_CATEGORY_MAP.items():
        for keyword in keywords:
            if keyword in message_lower:
                return category

    uncategorized_log.append(message)
    return "Uncategorized"

# Parses clang-tidy output and organizes issues by file and category.
def parse_clang_output(lines):
    issues_by_program = defaultdict(list) 
    current_issue = None # Tracks the current issue being processed.
    last_error_line = None # Tracks the last seen error/warning location.

    issue_pattern = re.compile(r'(.+?):(\d+):\d+: (error|warning): (.+)')
    note_pattern = re.compile(r'(.+?):(\d+):\d+: note: (.+)')

    for line in lines:
        line = line.strip()

        # Check if line matches an error/warning pattern.
        issue_match = issue_pattern.match(line)
        if issue_match:
            _, line_no, level, message = issue_match.groups()
            category = categorize(message) # Determine category for this issue.
            current_issue = {
                'line': int(line_no),
                'level': level,
                'message': message,
                'notes': []
            }

            last_error_line = int(line_no)
            issues_by_program[category].append(current_issue)
            continue

        # Check if line matches a note pattern (related to previous issue).
        note_match = note_pattern.match(line)
        if note_match:
            _, line_no, message = note_match.groups()
            line_no = int(line_no)
            if current_issue and last_error_line == line_no:
                current_issue['notes'].append(message)

    return issues_by_program, uncategorized_log