-- Wantedly のプロフィール閲覧（GQL node）の生データに近い中間テーブル
CREATE TABLE wantedly_profile_view_raw (
    id                      BIGSERIAL PRIMARY KEY,
    viewer_user_id          TEXT NOT NULL,  -- GQL node.userId
    viewer_company_page_url TEXT,           -- GQL node.companyPageUrl
    viewer_company_name_raw TEXT,           -- 生の会社名文字列
    viewed_at_raw           TEXT NOT NULL,  -- 生の日時文字列（例: "今日" "n日前"）
    viewed_at               TIMESTAMPTZ NOT NULL, -- パース済み日時
    raw_json                JSONB NOT NULL, -- 元の GQL node 全体
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT wantedly_profile_view_raw_uniq
        UNIQUE (viewer_user_id, viewed_at)
);

-- Wantedly 会社の識別キー（URL / slug）
CREATE TABLE wantedly_companies (
    id                  BIGSERIAL PRIMARY KEY,
    company_page_url    TEXT NOT NULL UNIQUE, -- 例: https://www.wantedly.com/companies/company_xyz
    company_slug        TEXT NOT NULL UNIQUE,                 -- 例: company_xyz
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 会社情報の追加属性（AI/手動など）
CREATE TYPE company_attribute_source AS ENUM ('ai'); -- 今は ai だけ 将来拡張可能

CREATE TABLE wantedly_company_attributes (
    id              BIGSERIAL PRIMARY KEY,
    company_id      BIGINT NOT NULL UNIQUE REFERENCES wantedly_companies(id),
    name            TEXT,
    domain          TEXT,
    source          company_attribute_source NOT NULL DEFAULT 'ai', -- 今は ai だけ 将来拡張可能
    confidence      NUMERIC(3,2),         -- 推定精度 0.00 〜 1.00
    CHECK (confidence IS NULL OR (confidence >= 0 AND confidence <= 1)), 
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Wantedly 上で識別できる閲覧ユーザー（ログインユーザーのみ）
CREATE TABLE wantedly_viewers (
    id              BIGSERIAL PRIMARY KEY,
    source_user_id  TEXT NOT NULL UNIQUE,                              -- node.userId
    company_id      BIGINT REFERENCES wantedly_companies(id), -- 現在の所属会社
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 正規化されたプロフィール閲覧インプレッション
CREATE TABLE wantedly_impressions (
    id                  BIGSERIAL PRIMARY KEY,
    viewer_id           BIGINT NOT NULL REFERENCES wantedly_viewers(id),
    company_id_at_view  BIGINT NOT NULL REFERENCES wantedly_companies(id),
    impressed_at        TIMESTAMPTZ NOT NULL,                  -- 確定済みの閲覧日時
    raw_profile_view_id BIGINT NOT NULL REFERENCES wantedly_profile_view_raw(id), -- 元 raw レコード
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX wantedly_impressions_company_time_idx
    ON wantedly_impressions (company_id_at_view, impressed_at DESC);

CREATE INDEX wantedly_impressions_viewer_time_idx
    ON wantedly_impressions (viewer_id, impressed_at DESC);

CREATE INDEX wantedly_impressions_impressed_at_idx
    ON wantedly_impressions (impressed_at);
