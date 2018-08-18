#!/usr/bin/env python
# -*- coding: utf-8 -*-
import curses
import locale
import time

import cgol

locale.setlocale(locale.LC_ALL, '')

PAUSE = 0.250

universe = cgol.Universe()

screen = curses.initscr()
curses.curs_set(0)
height, width = screen.getmaxyx()

def handle_screen_resize(screen, height, width):
    resize = curses.is_term_resized(height, width)
    if resize:
        height, width = screen.getmaxyx()
        screen.clear()
        curses.resizeterm(height, width)
        screen.refresh()
    return height, width


def display_render(screen, height, width, render):
    for i, row in list(enumerate(render.split()))[:height]:
        screen.addstr(i, 0, row[:width])
    screen.clrtoeol()
    screen.refresh()


while True:
    try:
        universe.tick()
        render = universe.render()

        height, width = handle_screen_resize(screen, height, width)
        display_render(screen, height, width, render)

        time.sleep(PAUSE)
    except KeyboardInterrupt:
        break

curses.endwin()
