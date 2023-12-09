import express, { NextFunction, Request, Response } from 'express';
import { engine } from 'express-handlebars';
import { PostgrestClient } from '@supabase/postgrest-js'
import path from 'path';

const app = express();
const port = process.env.PORT || 3000;

interface Registrant {
    name: string
}

const REST_URL = process.env["supabase_rest_url"]!;
const SUPABASE_API_KEY = process.env["supabase_api_key"]!;
const SUPABASE_TABLENAME = process.env["supabase_table_name"]!;
const postgrest = new PostgrestClient(REST_URL);
postgrest.headers['apikey'] = SUPABASE_API_KEY;

app.engine('handlebars', engine());
app.set('view engine', 'handlebars');
app.set('views', path.resolve(__dirname, "./views"));
app.use(express.json());
app.use(express.urlencoded());

app.get('/', (req: Request, res: Response) => {
    res.render('home');
});

app.post('/', async (req: Request, res: Response, next: NextFunction) => {
    try {
        console.log(req.body);
        if (req.body.name != null) {
            const name = (req.body as Registrant).name;
            const { data } = await postgrest.from(SUPABASE_TABLENAME).insert({name: name}).select();
            const created = data![0];
            res.render('home', {registered: "true", number: created.id, name: created.name});
        } else {
            res.render('home', {error: "Something went wrong! Please let Brad know!"});
        }
    } catch(e) {
        console.error(e);
        res.render('home', {error: "Something went wrong! Please let Brad know!"});
        next(e);
    } 
})
  
app.listen(port, () => {
    console.log(`Server running at http://localhost:${port}`);
});
  
