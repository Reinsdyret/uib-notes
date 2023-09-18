<!DOCTYPE html>

<html>
    <head>
        <title>INF115 - CE3</title>

        <!-- Some styling I added to the tables to make them look decent -->
        <style rel="stylesheet">
            table {
                border: 1px solid black;
            }

            th {
                border: 1px solid black;
                padding-right: 10px;
                padding-left: 10px;
            }

            td {
                text-align:center;
                border: 1px dotted black;
            }
        </style>

    </head>
    
    <body>
        <h1>INF115 - Compulsory exercise 3</h1>

            <?php
            /*
                Database configuration
            */

            // Connection parameters
            $host 		= 'localhost';
            $user 		= 'root';
            $password 	= '';
            $db 		= 'bysykkel';

            // Connect to the database
            $conn = mysqli_connect($host, $user, $password, $db);

            // Connection check
            if(!$conn) {
                exit('Error: Could not connect to the database.');
            }

            // Set the charset
            mysqli_set_charset($conn, 'utf8');
            ?>

        <h1> Task 1 </h1>
            <h2> a) </h2>
            <!-- Write your solution to 1a here -->

            <h2> b) </h2>
            <!-- Write your solution to 1b here -->
            
            <h2> c) </h2>
            <!-- Write your solution to 1c here -->
            
        
        <h1> Task 2 </h1>
            <h2> a) </h2>
            <?php
            try {
                $query = "UPDATE users SET name=\"Tore Antonsen\" WHERE user_id = 20";
                mysqli_query($conn, $query);

                $query = "UPDATE users SET user_id = 21 where user_id = 20";
                mysqli_query($conn, $query);

                echo "The queries ran with no problems";
            }

            catch(mysqli_sql_exception $e){
                echo 'Message: ' .$e->getMessage();
            }

            ?>

            <h2> b) </h2>
            <?php
            $query = "SELECT b.bike_id as Bike_id , b.NAME as Bike_name, s.NAME as Station_name
                        FROM bikes b
                        INNER JOIN stations s
                                ON s.station_id = b.station_id
                        WHERE b.status = \"active\"
                        ORDER BY b.bike_id";
            
            $result = mysqli_query($conn, $query);
            makeTable(["Bike_id", "Bike_name", "Station_name"], $result);
            ?>

            <h2> c) </h2>
            <?php
            $query = "SELECT Sum(table_rows) as totalRows
                        FROM information_schema.tables
                        WHERE table_schema = \"bysykkel\"";

            $result = mysqli_query($conn, $query);
            echo "Total number of rows in the bysykkel database is: ". $result["totalRows"];
            ?>

        <h1> Task 3 </h1>
            <h2> a) </h2>
            <?php
            $query = "SELECT u.NAME AS \"Name\",
                        s.start_time AS SubscriptionStartTime,
                        Count(*) AS NumberOfSubscriptions
                        FROM users u
                        INNER JOIN subscriptions s
                                ON u.user_id = s.user_id
                        WHERE Year(s.start_time) >= 2020
                        GROUP BY u.NAME";

            $result = mysqli_query($conn, $query);
            makeTable(["Name","SubscriptionStartTime","NumberOfSubscriptions"], $result)
            ?>

            <h2> b) </h2>
            <!-- Write your solution to 3b here -->

            <h2> c) </h2>
            <?php
            $query = "SELECT u.user_id as userID, u.NAME as userName,
                        Sum(CASE WHEN Year(s.start_time) = 2018 THEN 1 ELSE 0 END) AS \"2018\",
                        Sum(CASE WHEN Year(s.start_time) = 2019 THEN 1 ELSE 0 END) AS \"2019\",
                        Sum(CASE WHEN Year(s.start_time) = 2020 THEN 1 ELSE 0 END) AS \"2020\",
                        Sum(CASE WHEN Year(s.start_time) = 2021 THEN 1 ELSE 0 END) AS \"2021\"
                        FROM subscriptions s
                                INNER JOIN users u
                                        ON u.user_id = s.user_id
                        GROUP BY u.user_id";

            $result = mysqli_query($conn, $query);
            makeTable(["userID","userName","2018","2019","2020","2021"], $result);
            ?>
        
        <h1> Task 4 </h1>
        <form action="?" method="post">
            <label for="selectSubscription">Choose a subscription-type:</label>
            <select name="subscriptionType" id="selectSubscription">
                <option value="Day">Day</option>
                <option value="Week">Week</option>
                <option value="Month">Month</option>
                <option value="Year">Year</option>
            </select>
            <input type="submit">
        </form>

        <?php
        
        if(isset($_POST["subscriptionType"])){
            $type = $_POST["subscriptionType"];
            $query = "SELECT Sum(CASE WHEN type = \"$type\" THEN 1 ELSE 0 END) AS countSub, Count(*) AS totalRows FROM subscriptions";
            $result = mysqli_query($conn, $query);
            $row = mysqli_fetch_assoc($result);
            $percent = $row["countSub"]/$row["totalRows"]*100;

            echo "<table>";
            echo "<tr><th>SubscriptionType</th>";
            echo "<th># of Subscriptions</th>";
            echo "<th>% of all Subscriptions</th></tr>";

            echo "<tr><td>".$type."</td>";
            echo "<td>". $row["countSub"]. "</td>";
            echo "<td>". round($percent) . "%</td></tr>";

            echo "</table>";
        }else{
            echo "Choose a type";
        }
        
        ?>


    </body>
</html>

<?php
// This is just my function for making tables with data given from an sql query

/**
 * 
 * Generate a html table with the given headears and sql result
 * Note that the headers in the headers param must be similar to the column names of the sql result
 * 
 * @param headers an array of table headers(strings)
 * @param sqlResult a completed sql query
 */
function makeTable($headers, $sqlResult){
    echo "<table>";
    echo "<tr>";
    foreach($headers as $header){
        echo "<th>". $header. "</th>";
    }
    echo "</tr>";

    while($row = mysqli_fetch_assoc($sqlResult)){
        echo "<tr>";

        foreach($headers as $header){
            echo "<td>".$row[$header]. "</td>";
        }

        echo "</tr>"; 
    }

    echo "</table>";
}
?>
