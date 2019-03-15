# backupr19
custom backup tool written in rust

Add a config.txt to the root. Format should be line 1: full path of original directory to be backed up; line 2: full path to folder to contain the backup; e.g.

c:\\users\\user1\\mywork
e:\\mywork

Note that the backup directory must already exist.

Caution: the programme will delete files and directories in the backup directory if they have been deleted or do not exist in the original. You must be careful when writing your config.txt that you do not reverse the order - this may result in the deletion of your original data.
