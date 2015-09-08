<?php
/**
 * Created by PhpStorm.
 * User: nicolas
 * Date: 26/08/15
 * Time: 16:26
 */

define('MAX_FILE_SIZE', 40000000);
define('SPLITTED_FILES_PREFIX', 'temp_mbox_');

if(isset($argv[1]) && !empty($argv[1]))
    $mbox_input_file = $argv[1];
else die('No mbox entry file specified');

if(isset($argv[2]) && !empty($argv[2]))
    $mbox_output_folder = $argv[2];
else die('No output folder specified');


if(!file_exists($mbox_output_folder))
    mkdir($mbox_output_folder);

$handle = fopen($mbox_input_file, "r") or die("Couldn't get handle");
if ($handle) {
    $mail_counter = 1;
    $big_mails_counter = 0;
    $file_counter = 0;
    $i= 0;
    $current_mail = "";

    $current_mail_file_path = $mbox_output_folder . '/' . SPLITTED_FILES_PREFIX . $file_counter . ".mbox";

    while (!feof($handle)) {
        $buffer = fgets($handle);
        if(substr($buffer, 0, 5) == "From ") {

            // Handle end of previous mail
            if(!empty($current_mail)) {
                // Reach of the file limit.

                if(file_exists($current_mail_file_path)) {
                    $file_size = filesize($current_mail_file_path);
                } else $file_size = 0;

                $mail_size = strlen($current_mail);

                // Handle very big mails not in classic stream
                if($mail_size > MAX_FILE_SIZE) {
                    $big_mails_counter++;
                    file_put_contents("$mbox_output_folder/big_mail_$big_mails_counter.mbox", $current_mail, FILE_APPEND);
                }
                // File too big with current mail
                else if(($file_size + $mail_size) > MAX_FILE_SIZE) {
                    // First mail too big
                    if($mail_counter == 1)
                        die("First mail too big to store\n");

                    // Start new one
                    else {
                        echo "\t- Start new mail file\n";
                        $file_counter++;
                        $current_mail_file_path = sprintf("$mbox_output_folder/%s$file_counter.mbox", SPLITTED_FILES_PREFIX);
                        file_put_contents($current_mail_file_path, $buffer, FILE_APPEND);
                    }
                }
                // put in current file
                else {
                    echo "\t- Write in current mail file\n";
                    file_put_contents($current_mail_file_path, $current_mail, FILE_APPEND);
                    echo "\t- File size : $file_size bytes\n";
                }
                $current_mail = "";
            }

            echo "Mail #$mail_counter\n";
            $mail_counter++;
        }
        $current_mail .= $buffer;
        $i++;
        clearstatcache();
    }
    fclose($handle);
}