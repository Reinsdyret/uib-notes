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


ui.panel_title("INF115 VT24: Bysykkel", "Bysykkel")

with ui.card():
    ui.card_header("Task 1")
    
    with ui.card():
        ui.card_header("a)")
        #--- Write your solution to 1a here ---
        
        @render.text()
        def rndr_name():
            return 'Name, student_id'

    with ui.card():
        ui.card_header("b)")
        #--- Write your solution to 1b here ---
        
        ui.input_text("name", "Name:", "")
        ui.input_text("mail", "Mail:", "")
        ui.input_text("phone", "Phone number:", "")
        
        ui.input_action_button("submit", "Submit", width='200px', class_="btn-success")
        
        @render.text
        @reactive.event(input.submit)
        def rndr_form_name():
            return input.name()
        
        @render.text
        @reactive.event(input.submit)
        def rndr_form_mail():
            return input.mail()
        @render.text
        @reactive.event(input.submit)
        def rndr_form_phone():
            return input.phone()
          
    with ui.card():
        ui.card_header("c)")
        #--- Write your solution to 1c here ---
        
        @render.text
        @reactive.event(input.submit)
        def check_form_name():
            res = input.name()
            if re.fullmatch(r'[A-Za-zåÅøØæÆ]+', res):
                res += ' - Valid'
            else:
                res += ' - Not valid'
            return res
        
        @render.text
        @reactive.event(input.submit)
        def check_form_mail():
            res = input.mail()
            if re.search(r'@', res):
                res += ' - Valid'
            else:
                res += ' - Not valid'
            return res
    
        @render.text
        @reactive.event(input.submit)
        def check_form_phone():
            res = input.phone()
            if res.isnumeric() and len(res) == 8:
                res += ' - Valid'
            else:
                res += ' - Not valid'
            return res

with ui.card():
    ui.card_header("Task 2")
    
    with ui.panel_well():
        ui.card_header("a)")
        #--- Write your solution to 2a here ---
        
        @render.table(border=1, justify='left')
        def rndr_task2a():
            return pd.read_sql_query("SELECT name FROM users ORDER BY name;", con)
        
    with ui.panel_well():
        ui.card_header("b)")
        #--- Write your solution to 2b here ---
        
        @render.table(border=1, justify='left')
        def rndr_task2b():
            return pd.read_sql_query("SELECT DISTINCT name, status FROM bikes;", con)
        
    with ui.panel_well():
        ui.card_header("c)")
        #--- Write your solution to 2c here ---
        
        @render.table(border=1, justify='left')
        def rndr_task2c():
            return pd.read_sql_query("SELECT type, count(*) AS Purchased FROM subscriptions GROUP BY type;", con)
        
with ui.card():
    ui.card_header("Task 3")
    
    with ui.panel_well():
        ui.card_header("a)")
        #--- Write your solution to 3a here ---
        
        ui.input_text("filter_name", "Name:", "", width='200px')
        ui.input_action_button("filter", "Filter", width='200px', class_="btn-success")
        
        @render.table(border=1, justify='left')
        @reactive.event(input.filter, ignore_none=False)
        def filter_users():
            return pd.read_sql_query("SELECT user_id, name, printf('%08d', phone_number) AS phone FROM users WHERE name LIKE ?;", con, params=['%'+input.filter_name()+'%'])


    with ui.panel_well():
        ui.card_header("b)")
        #--- Write your solution to 3b here ---
        
        @render.table(border=1, justify='left')
        def rndr_task3b():
            return pd.read_sql_query("SELECT S.station_id, S.name, COUNT(T.trip_id) AS \"Trips\" FROM trips AS T, stations as S WHERE T.end_station = S.station_id GROUP BY S.name;", con)

        
    with ui.panel_well():
        ui.card_header("c)")
        #--- Write your solution to 3c here ---
        
        @render.table(border=1, justify='left')
        def rndr_task3c():
            return pd.read_sql_query('''SELECT users.user_id, users.name,
                count(case strftime('%Y', subscriptions.start_time) when '2018' then 1 else null end) as '2018',
                count(case strftime('%Y', subscriptions.start_time) when '2019' then 1 else null end) as '2019',
                count(case strftime('%Y', subscriptions.start_time) when '2020' then 1 else null end) as '2020',
                count(case strftime('%Y', subscriptions.start_time) when '2021' then 1 else null end) as '2021'
                FROM users INNER JOIN subscriptions on users.user_id = subscriptions.user_id GROUP BY users.user_id;
                ''', con)

with ui.panel_well():
    ui.card_header("Task 4")
    #--- Write your solution to 4 here ---
    
    _ = cur.execute('SELECT station_id, name FROM stations ORDER BY name;')
    
    stations = {row[0]: row[1] for row in cur.fetchall()}
    
    ui.input_selectize('stations', 'Choose a station:', stations, width='400px')
    ui.input_switch('active_trip', 'Active trip', False)
    
        
    @render.table(border=1, justify='left',render_links=True, escape=False)
    def rndr_tast4():
        df = pd.read_sql_query('SELECT * FROM stations WHERE station_id = ?;', con, params=[input.stations()])
    
        if input.active_trip():
            df['Availability'] = df['available_spots'] / df['max_spots']
        else:
            df['Availability'] = (df['max_spots'] - df['available_spots']) / df['max_spots']
            
        df['Availability'] = round(df['Availability'] * 100, 0).map('{:0.0f}'.format) + '%'
        
        df['Link'] = df.apply(lambda x: f'<a href="https://www.google.com/maps?q={x.latitude},{x.longitude}">Link</>', axis=1)

        return df[['name', 'Availability', 'Link']]
    

    