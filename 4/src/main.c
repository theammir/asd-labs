#include <curses.h>
#include <unistd.h>

const int REFRESH_DELAY = 5000;

void get_symbol(int *symbol) {
	printw("Enter the symbol and press Enter:");
	
	// Ask for the symbol until Enter is pressed
	int next_symbol;
	for (;;) {
		next_symbol = getch();
		if (next_symbol == KEY_ENTER || next_symbol == '\n' || next_symbol == '\r') {
			break;
		} else if (next_symbol == KEY_BACKSPACE) {
			mvprintw(0, 34, "  ");
		} else {
			mvprintw(0, 34, "%c ", next_symbol);
			*symbol = next_symbol;
		}
	}
}

void draw_loop(int height, int width, int symbol) {
	const char WIDTH_IS_ODD = width % 2 == 1;

	char direction = -1;
	for (
		int x = 0.5*width - 1 + WIDTH_IS_ODD; // middle
		(x < width) && (x >= 0); // out of bounds check
		x--
	) {
		for (
			int y = (direction == 1) ? 0 : height - 1; // whether to start from top or bottom
			(y < height) && (y >= 0); // out of bounds check
			y += direction
		) {
			mvaddch(y, x, symbol); // left point
			refresh();
			usleep(REFRESH_DELAY);

			// there was a check not to draw the second point if width is odd and x == 0,
			// but mvaddch macro already does that for me
			
			mvaddch(y, width - x - 1 + WIDTH_IS_ODD, symbol); // right point, mirrored
			refresh();
			usleep(REFRESH_DELAY);
		}
		direction *= -1;
	}
}

int main(void) {
	initscr();
	noecho();
	keypad(stdscr, TRUE);
	curs_set(FALSE);

	int height, width;
	getmaxyx(stdscr, height, width);
	
	int symbol;
	get_symbol(&symbol);

	clear();
	refresh();

	draw_loop(height, width, symbol);

	getch();
	endwin();
	return 0;
}

// vim: ts=4: sw=4:
