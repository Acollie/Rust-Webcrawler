# Rust-Webcrawler

This is a web crawler written in rust. It visits sites and compiles a list of links. The aim being for them to be indexed as part of a search engine.

## Todo

--- 
### Ingestion engine
A system to be able to evaluate
#### Parameters
- Page title
- Hits (pages linking)
- Most often mentioned useful word

Remove similar pages.

--- 
### Configuration file
A file which allows for the user to settings before start.

---

### Database Integration
Allowing after a sweep for the data to be added to database.
With configuration file, for MYSQL, with Postgres support later.
---
### Multithreaded
Create allow for the program to be able to run on multiple threads in CPU allows.