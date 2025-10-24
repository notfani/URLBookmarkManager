FROM postgres:14

RUN apt-get update && apt-get install -y locales && \
    localedef -i ru_RU -c -f UTF-8 -A /usr/share/locale/locale.alias ru_RU.UTF-8 && \
    rm -rf /var/lib/apt/lists/*

ENV LANG ru_RU.utf8

COPY init-scripts/ /docker-entrypoint-initdb.d/

RUN echo "shared_buffers = 256MB" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "effective_cache_size = 1GB" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "maintenance_work_mem = 64MB" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "checkpoint_completion_target = 0.9" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "wal_buffers = 16MB" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "default_statistics_target = 100" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "random_page_cost = 1.1" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "effective_io_concurrency = 200" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "work_mem = 4MB" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "min_wal_size = 1GB" >> /usr/share/postgresql/postgresql.conf.sample && \
    echo "max_wal_size = 4GB" >> /usr/share/postgresql/postgresql.conf.sample

EXPOSE 5432

