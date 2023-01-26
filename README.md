# Rustracer
Me having fun creating a simple ray tracer with scene editor in Rust


I want to make it multi-threaded so that each thread independently compute colors of a part of the image. See the following picture, each colored band represents the region each thread will be handling :
![image](https://user-images.githubusercontent.com/20617764/214865851-f975da1f-91a6-4628-855b-b87eb82e7f31.png)
