# 22/08/24

- |17:00 - 19:00| Create data models for roadmaps and user
- |17:00 - 19:00| create workflow
- |17:00 - 19:00| set up database for the application

# 23/08/24

- |08:20 - 10:20| create tables according to the models

# 24/08/24

- create APIs accoring to the workflow
- priority (Roadmap API > Auth API > Other APIs)
- |20:00 - 23:00| POST /map - for creating a new roadmap

# 25/08/24

(Rust is absolutely killing me, its taking so much time lol)

- |16:00 - 17:00| PUT /map/:mapid/edit - for updating a roadmap
- |17:00 - 18:00| POST /login
- |18:00 - 18:40| POST /register

# 31/08/24

- 1. Integrate create map page
  - 1. Create form t create a map - title, description, keywords, tags, sources.
  - TODO: FIXES:
    1.

# 01/09/24

- 2. Integrate edit map page
- 3. Show tags, username for maps in `/roadmaps` page
- 4. Show proper time for roadmaps

# 02/09/24 - 09/09/24

- 5. Build map page
  - Done building the basic map (08/09/24) Done
  - Make it dynamic for all map sizes Done
  - add buttons to zoom in and out (done zoom on scoll will add buttons later)
- 6. Create a multiselect drop down for the tags in filter options in `/roadmaps` - Done with a basic one, need more styling
- 7. Add nanoid to slug
- 8. Add unique enough Ids for nodes in reactflow (serial ids get conflicted during editing) or set new Ids to start from max pre existing id + 1 to avoid id conflicts - Done (using max serial id now)
