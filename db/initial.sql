CREATE TABLE public.users (
    id uuid NOT NULL,
    email character varying(100) NOT NULL,
    name character varying(150) NOT NULL,
    password character varying(150) NOT NULL,
    created_at timestamp without time zone DEFAULT timezone('UTC'::text, now()) NOT NULL,
    updated_at timestamp without time zone,
    role character varying(20) NOT NULL
);
ALTER TABLE public.users OWNER TO demo;
ALTER TABLE ONLY public.users ADD CONSTRAINT users_email_key UNIQUE (email);
ALTER TABLE ONLY public.users ADD CONSTRAINT users_pkey PRIMARY KEY (id);

CREATE TABLE public.articles (
    id uuid NOT NULL,
    title character varying(200) NOT NULL,
    content text,
    created_at timestamp without time zone DEFAULT timezone('UTC'::text, now()) NOT NULL,
    updated_at timestamp without time zone,
    tags character varying(500),
    url character varying(200),
    in_home bool NOT NULL DEFAULT false
);
ALTER TABLE public.articles OWNER TO demo;
ALTER TABLE ONLY public.articles ADD CONSTRAINT articles_pkey PRIMARY KEY (id);

CREATE TABLE public.comments (
    id uuid NOT NULL,
    author character varying(100) NOT NULL,
    email character varying(100) NOT NULL,
    article_id uuid NOT NULL,
    content text,
    created_at timestamp without time zone DEFAULT timezone('UTC'::text, now()) NOT NULL,
    updated_at timestamp without time zone
);
ALTER TABLE public.comments OWNER TO demo;
ALTER TABLE ONLY public.comments ADD CONSTRAINT comments_pkey PRIMARY KEY (id);
