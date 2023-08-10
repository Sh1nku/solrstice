import pickle

from solrstice.group import GroupingComponent
from solrstice.queries import (
    CommitType,
    DeleteQueryBuilder,
    SelectQueryBuilder,
    UpdateQueryBuilder,
)


def test_pickle_works_select_query_builder():
    builder = SelectQueryBuilder(
        fq=["test", "test"],
        grouping=GroupingComponent(fields=["test"], main=True, facet=False),
    )
    string = pickle.dumps(builder)
    builder_copy: SelectQueryBuilder = pickle.loads(string)
    assert builder_copy.fq == builder.fq
    assert builder_copy.grouping.fields == builder.grouping.fields
    assert builder_copy.grouping.main == builder.grouping.main
    assert builder_copy.grouping.facet == builder.grouping.facet


def test_pickle_works_update_query_builder():
    builder = UpdateQueryBuilder(handler="test", commit_type=CommitType.Soft)
    string = pickle.dumps(builder)
    builder_copy: UpdateQueryBuilder = pickle.loads(string)
    assert builder_copy.handler == builder.handler
    assert builder_copy.commit_type == builder.commit_type


def test_pickle_works_delete_query_builder():
    builder = DeleteQueryBuilder()
    string = pickle.dumps(builder)
    builder_copy: DeleteQueryBuilder = pickle.loads(string)
    assert builder_copy.handler == builder.handler
