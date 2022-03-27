# A short explanation

[How to download section](https://github.com/Grrwahrr/avtomat_dos#how-to-download) | [Guide for Windows](assets/howto/WINDOWS.md) | [Guide for macOS](assets/howto/MACOS.md)   


This program visits various russian websites in order to keep the servers under load. The idea is that many visitors (as in thousands) can overwhelm a server.
This program is sort of a normal web browser, except that it is fully automated and that it never displays the websites.  

#### What websites does it visit?

The [IT Army of Ukraine](https://t.me/itarmyofukraine2022) occasionally publishes a list of internet services in russia that they consider of interest. This program automatically downloads a list of targets and starts visiting the websites.

#### Why would I use this?

If you know how to use _hping_ or _slowhttptest_, you do not need this program. This program is purely for normal people that are interested in using some of their internet capacity to put strain on russian webservers.

#### How does it work?

You download the executable and press the start button. Depending on the intensity setting, configured with the slider, the program will generate more or less traffic. Your internet may slow down if you choose a high intensity.

#### How does this work in detail?

The program works much like your web browser. It sends completely normal HTTP requests to the web server and loads various resources linked on the page, such as images. Unlike your browser, it will never display the data that was received but simply discard it. Thus, your computer does not waste processing time displaying the website.  
All servers have a finite capacity to transmit data. While 1 or even 100 users are not able to overwhelm a server, all servers will eventually succumb if put under too much strain.  
Experienced server operators can protect themselves from individuals who send too many requests or open too many connections from the same source. It is however much harder to protect against thousands of legit web requests.

#### Is it safe for me to use?

Again, this is close to a normal web browser, however it will visit the same website again and again over time. More than a normal user would. It seems reasonable to assume that you won't get into trouble for browsing the internet. As far as your internet provider is concerned, this is about as interesting to them as you downloading a large file over a long time.  
If you are located in russia however, you may not be very safe to begin with, and you may want to be extra cautious.  
  
If you do not have an internet flat rate and pay for the data you receive, for instance if you use a mobile internet connection, you may not want to use this program as it will, by design, receive quite a lot of data.

### How to download

[Guide for Windows](assets/howto/WINDOWS.md)  
[Guide for macOS](assets/howto/MACOS.md)