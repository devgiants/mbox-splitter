#MBOX Splitter
Here is a script to allow splitting large .mbox files, such as ones provided by Google Apps when exporting Apps accounts mailboxes

## Configuration
Dead simple, just change constants in top of file. 2 so far :

 - **MAX_FILE_SIZE** : set the max file size for each of the split files (in bytes)
 - **SPLITTED_FILES_PREFIX** : set the prefix you want for splitted files naming.

## Usage
Also dead simple :

    > php mbox_splitter.php /link/to/big/mailbox.mbox /dest/folder/for/splitted

All log/echo is so far in current console.

