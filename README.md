# rsdedupe

Same deal as [goshutter](https://github.com/justnat3/goshutter), just severely upgraded. I wanted to add a gui, make it a little faster- start using rust more. 
With that out of the way the basic idea of this application is to take a directory and the file in it and run a deduplication routine on them. Taking SHA256 and making
a digest of the contents in the file to then compare it again other files. If we get a hit then we can move the duplicated file to a dupelication directory to be manually
reviewed by the user. 

The gui is in no place where I want to show it off, however it is written in egui(love this project). It will be functional but I am working out the kinks :D 


Cheers,
-Nate
