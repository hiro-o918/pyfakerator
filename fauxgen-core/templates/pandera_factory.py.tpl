class {{ record_class_name }}(TypedDict):
    """A data structure representing {{ dataframe_schema_class_name }} entries."""
{%- for field in fields %}
    {{ field.name }}: {{ field.get_type_annotation() }}
{%- endfor %}


def {{ record_factory_name }}(
    *,
{%- for field in fields %}
    {{ field.name }}: {{ field.get_type_annotation() }} | f.Unset = f.UNSET,
{%- endfor %}
    seed_: int | None = None,
) -> {{ record_class_name }}:
    """Creates a mock {{ dataframe_schema_class_name }} entry with randomized values.

    Each field is generated with appropriate constraints and validation rules.
    Values can be overridden by providing specific field values.

    Args:
{%- for field in fields %}
        {{ field.name }} ({{ field.get_type_annotation() }}): {{ field.get_description() }}
{%- endfor %}
        seed_ (int | None): Seed value for deterministic data generation.
                            The same seed will always produce the same values.

    Returns:
        {{ record_class_name }}: A new mock entry with generated data.
    """
    return {
{%- for field in fields %}
        "{{ field.name }}": f.Unset.unwrap_or_else({{ field.name }}, lambda: {{ field.get_faker_method() }}),
{%- endfor %}
    }

