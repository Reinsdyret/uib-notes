import sqlite3
import pandas as pd
import re

from shiny import reactive
from shiny.express import render, ui, input



# creates the database bysykkel
with open('bysykkel.sql', 'r') as sql_file:
    sql_script = sql_file.read()

con = sqlite3.connect(':memory:')
cur = con.cursor()
_ = cur.executescript(sql_script)


with ui.card():
    ui.card_header("Task 1")
    
    with ui.card():
        ui.card_header("a)")
        #--- Write your solution to 1a here ---


    with ui.card():
        ui.card_header("b)")
        #--- Write your solution to 1b here ---
        
        
    with ui.card():
        ui.card_header("c)")
        #--- Write your solution to 1c here ---
        

with ui.card():
    ui.card_header("Task 2")
    
    with ui.panel_well():
        ui.card_header("a)")
        #--- Write your solution to 2a here ---
        

    with ui.panel_well():
        ui.card_header("b)")
        #--- Write your solution to 2b here ---
        
        
    with ui.panel_well():
        ui.card_header("c)")
        #--- Write your solution to 2c here ---
        
        
with ui.card():
    ui.card_header("Task 3")
    
    with ui.panel_well():
        ui.card_header("a)")
        #--- Write your solution to 3a here ---
        

    with ui.panel_well():
        ui.card_header("b)")
        #--- Write your solution to 3b here ---
        
        
    with ui.panel_well():
        ui.card_header("c)")
        #--- Write your solution to 3c here ---
        
        
with ui.panel_well():
    ui.card_header("Task 4")
    #--- Write your solution to 4 here ---
    